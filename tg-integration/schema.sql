CREATE TABLE IF NOT EXISTS listener (
    flow_id text NOT NULL,
    flows_user text NOT NULL,
    bot_token text NOT NULL,
    handler_fn text,
    PRIMARY KEY (flow_id)
);

