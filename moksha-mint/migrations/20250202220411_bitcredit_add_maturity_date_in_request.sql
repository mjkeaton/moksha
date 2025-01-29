ALTER TABLE bitcredit_requests_to_mint
    ADD COLUMN IF NOT EXISTS maturity_date BIGINT;