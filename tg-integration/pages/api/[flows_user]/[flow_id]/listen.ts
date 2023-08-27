import crypto from "crypto";
import bs58 from 'bs58';
import type { NextApiRequest, NextApiResponse } from "next"
import { encrypt } from "@/lib/token";
import { pool } from '@/lib/pg';

const CALLBACK_URL = `${process.env.CODE_FLOWS_URL}/hook/telegram_on_deploy_handler/message`;

const fn = async (req: NextApiRequest, res: NextApiResponse) => {
    const { flows_user: flowsUser, flow_id: flowId, token, handler_fn: handlerFn } = req.query;

    if (!flowsUser || !flowId || !token || !handlerFn) {
        return res.status(400).send("Bad request");
    }

    if (typeof flowsUser != "string"
        || typeof flowId != "string"
        || typeof token != "string"
        || typeof handlerFn != "string"
    ) {
        return res.status(400).send("Bad request");
    }

    try {
        let iv = crypto.randomBytes(16);
        let enc = encrypt(token, iv);

        const st_json = JSON.stringify(enc);
        const st_bytes = new TextEncoder().encode(st_json);
        const st = bs58.encode(st_bytes);

        let listener = await pool.query("SELECT flows_user, flow_id, bot_token FROM listener where flow_id = $1", [flowId]);
        if (listener.rows && listener.rows.length > 0) {
            // As flow_id is the primary key, there should be only one record
            const oldToken = listener.rows[0].bot_token;
            if (oldToken === token) {
                // Has listened
                return res.status(200).json({});
            } else {
                // Revoke old listener for this flow
                const url = `https://api.telegram.org/bot${oldToken}/setWebhook?url=`;
                await fetch(url);
            }
        }

        // Register webhook for the token
        const url = `https://api.telegram.org/bot${token}/setWebhook?url=${CALLBACK_URL}&secret_token=${st}`;
        let resp = await fetch(url);

        if (resp.ok) {
            await pool.query(`
                INSERT INTO listener (flows_user, flow_id, bot_token, handler_fn)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (flow_id)
                DO UPDATE SET bot_token = excluded.bot_token,
                handler_fn = excluded.handler_fn
            `, [flowsUser, flowId, token, handlerFn]);
            
            return res.status(200).json({});
        } else {
            return res.status(400).send("invalid token");
        }

    } catch (e: any) {
        return res.status(500).end(e.toString());
    }
}

export default fn;
