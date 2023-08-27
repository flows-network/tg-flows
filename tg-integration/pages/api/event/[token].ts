import type { NextApiRequest, NextApiResponse } from "next"
import bs58 from "bs58";
import { decrypt } from "@/lib/token";
import { pool } from '@/lib/pg';

const fn = async (req: NextApiRequest, res: NextApiResponse) => {
    const { token } = req.query;

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

        let listener = await pool.query("SELECT flows_user, flow_id, handler_fn FROM listener where bot_token = $1", [dec_token]);
        if (listener.rows && listener.rows.length > 0) {
            let flowArray = [];
            for (let i = 0; i < listener.rows.length; i++) {
                flowArray.push({
                    flows_user: listener.rows[i].flows_user,
                    flow_id: listener.rows[i].flow_id,
                    handler_fn: listener.rows[i].handler_fn,
                });
            }

            return res.status(200).json(flowArray);
        } else {
            return res.status(404).send("No flow binding with the address");
        }
    } catch (e: any) {
        return res.status(500).send(e.toString());
    }
}

export default fn;
