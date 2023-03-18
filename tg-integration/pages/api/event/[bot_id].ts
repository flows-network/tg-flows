import type { NextApiRequest, NextApiResponse } from "next"
import { redis } from "@/lib/upstash";
import { decrypt } from "@/lib/token";
import bs58 from "bs58";

const fn = async (req: NextApiRequest, res: NextApiResponse) => {
    const { bot_id: token } = req.query;

    if (!token) {
        return res.status(400).send("Bad request");
    }

    if (typeof token != "string") {
        return res.status(400).send("Bad request");
    }

    try {
        const st = bs58.decode(token);
        const st_bytes = new TextDecoder().decode(st);
        const st_json = JSON.parse(st_bytes);

        const data = st_json.data;
        const iv = st_json.iv;

        const dec_token = decrypt(data, Buffer.from(iv, "base64"))

        let allFlows = await redis.hgetall(`telegram:${dec_token}:trigger`);

        if (allFlows) {
            let flowArray = [];
            for (let flows in allFlows) {
                let flow_id: any = allFlows[flows];
                flowArray.push({
                    flows_user: flows,
                    flow_id: flow_id,
                });
            }

            return res.status(200).json(flowArray);
        } else {
            return res.status(404).send("No flow binding with the address")
        }
    } catch (e: any) {
        return res.status(500).send(e.toString());
    }
}

export default fn;
