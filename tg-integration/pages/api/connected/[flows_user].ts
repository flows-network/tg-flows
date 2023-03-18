import type { NextApiRequest, NextApiResponse } from "next"
import { redis } from "@/lib/upstash";

const fn = async (req: NextApiRequest, res: NextApiResponse) => {
    const { flows_user } = req.query;

    if (!flows_user) {
        return res.status(400).send("Bad request");
    }

    if (typeof flows_user != "string") {
        return res.status(400).send("Bad request");
    }

    try {
        let results = [];
        let tokens = await redis.hgetall(`telegram:${flows_user}:listen`);

        for (const flow_id in tokens) {
            let url = `https://api.telegram.org/bot${tokens[flow_id]}/getMe`;
            let resp = await fetch(url);
            let json = await resp.json();

            let result = json["result"];
            let first_name = result["first_name"];

            results.push({
                name: first_name,
            });
        }

        return res.status(200).json({
            title: "Connected Bot",
            list: results
        });
    } catch (e: any) {
        return res.status(500).send(e.toString());
    }
}

export default fn;
