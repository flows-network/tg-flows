import crypto from "crypto";
import bs58 from 'bs58';
import type { NextApiRequest, NextApiResponse } from "next"
import { redis } from "@/lib/upstash";
import { encrypt } from "@/lib/token";

const CALLBACK_URL = `${process.env.CODE_FLOWS_URL}/hook/telegram/message`;

const fn = async (req: NextApiRequest, res: NextApiResponse) => {
    const { flows_user, flow_id, token } = req.query;

    if (!flows_user || !flow_id || !token) {
        return res.status(400).send("Bad request");
    }

    if (typeof flows_user != "string"
        || typeof flow_id != "string"
        || typeof token != "string"
    ) {
        return res.status(400).send("Bad request");
    }

    let iv = crypto.randomBytes(16);
    let enc = encrypt(token, iv);

    const st_json = JSON.stringify(enc);
    const st_bytes = new TextEncoder().encode(st_json);
    const st = bs58.encode(st_bytes);

    const oldToken = await redis.hget(`telegram:${flows_user}:listen`, flow_id);

    if (oldToken == token) {
        return res.status(200).json({});
    }

    // Revoke old listener for this flow
    if (oldToken) {
        await redis.hdel(`telegram:${oldToken}:trigger`, flow_id);
        const url = `https://api.telegram.org/bot${oldToken}/setWebhook?url=`;
        await fetch(url);
    }

    const url = `https://api.telegram.org/bot${token}/setWebhook?url=${CALLBACK_URL}&secret_token=${st}`;
    let resp = await fetch(url);

    if (resp.ok) {
        const pipe = redis.pipeline();
        pipe.hset(`telegram:${token}:trigger`, {
            [flow_id]: flows_user,
        });
        pipe.hset(`telegram:${flows_user}:listen`, {
            [flow_id]: token
        });
        await pipe.exec();

        return res.status(200).json({});
    } else {
        return res.status(400).send("invalid token");
    }

}

export default fn;
