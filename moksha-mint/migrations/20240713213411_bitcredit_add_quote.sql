CREATE TABLE IF NOT EXISTS bitcredit_requests_to_mint (
    bill_id TEXT NOT NULL PRIMARY KEY,
    bill_key TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bitcredit_mint_quotes (
    id UUID NOT NULL,
    bill_id TEXT NOT NULL PRIMARY KEY,
    node_id TEXT NOT NULL
);