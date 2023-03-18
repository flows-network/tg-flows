import type { NextApiRequest, NextApiResponse } from "next"
import { redis } from "@/lib/upstash";

const fn = async (req: NextApiRequest, res: NextApiResponse) => {
    const { flows_user, flow_id, token } = req.query;

    if (!flows_user || !flow_id || !token) {
        return res.status(400).send("Bad request");
    }

    if (typeof flows_user != "string"
        || typeof flow_id != "string"
        || typeof token != "string") {
        return res.status(400).send("Bad request");
    }

    const pipe = redis.pipeline();
    pipe.hdel(`telegram:${token}:trigger`, flow_id);
    pipe.hdel(`telegram:${flows_user}:listen`, flow_id);
    await pipe.exec();

    return res.status(200).send("ok");
}

export default fn;
