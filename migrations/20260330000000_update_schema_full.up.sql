-- Migration to update assets and add transactions table
ALTER TABLE assets ADD COLUMN IF NOT EXISTS user_id BIGINT REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE assets ADD COLUMN IF NOT EXISTS ticker TEXT;
ALTER TABLE assets ADD COLUMN IF NOT EXISTS asset_type TEXT DEFAULT 'Stocks';
ALTER TABLE assets ADD COLUMN IF NOT EXISTS quantity DOUBLE PRECISION DEFAULT 1.0;
ALTER TABLE assets ADD COLUMN IF NOT EXISTS avg_price DOUBLE PRECISION DEFAULT 0.0;
ALTER TABLE assets ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT NOW();

-- Drop unique constraint on name if it exists, so different users can own the same ticker
ALTER TABLE assets DROP CONSTRAINT IF EXISTS assets_name_key;

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    asset_name TEXT NOT NULL,
    ticker TEXT NOT NULL,
    tx_type TEXT NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    total_value DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'Completed',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
