use std::convert::Infallible;

use axum::extract::FromRequestParts;
use sqlx::{PgPool, Row};

use crate::{
    app::AppState,
    models::{Asset, UserRecord},
};

pub struct Repository {
    pub db: PgPool,
}

impl Repository {
    pub async fn ensure_schema(&self) -> Result<(), sqlx::Error> {
        // Guarantee tables exist and columns are present even before manual migration
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id BIGSERIAL PRIMARY KEY NOT NULL,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS assets (
                id BIGSERIAL PRIMARY KEY NOT NULL,
                user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
                ticker TEXT,
                name TEXT NOT NULL,
                asset_type TEXT DEFAULT 'Stocks',
                quantity DOUBLE PRECISION DEFAULT 1.0,
                unit_value DOUBLE PRECISION NOT NULL,
                avg_price DOUBLE PRECISION DEFAULT 0.0,
                created_at TIMESTAMPTZ DEFAULT NOW()
            );

            ALTER TABLE assets ADD COLUMN IF NOT EXISTS user_id BIGINT REFERENCES users(id) ON DELETE CASCADE;
            ALTER TABLE assets ADD COLUMN IF NOT EXISTS ticker TEXT;
            ALTER TABLE assets ADD COLUMN IF NOT EXISTS asset_type TEXT DEFAULT 'Stocks';
            ALTER TABLE assets ADD COLUMN IF NOT EXISTS quantity DOUBLE PRECISION DEFAULT 1.0;
            ALTER TABLE assets ADD COLUMN IF NOT EXISTS avg_price DOUBLE PRECISION DEFAULT 0.0;
            ALTER TABLE assets ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT NOW();
            ALTER TABLE assets DROP CONSTRAINT IF EXISTS assets_name_key;

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
            "#,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn list_assets(&self) -> Result<Vec<Asset>, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            r#"
            SELECT id, user_id, ticker, name, asset_type, quantity, unit_value, avg_price
            FROM assets
            ORDER BY id ASC;
            "#,
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn list_assets_by_user(&self, user_id: i64) -> Result<Vec<Asset>, sqlx::Error> {
        let assets = sqlx::query_as::<_, Asset>(
            r#"
            SELECT id, user_id, ticker, name, asset_type, quantity, unit_value, avg_price
            FROM assets
            WHERE user_id = $1 OR user_id IS NULL
            ORDER BY id ASC;
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        if assets.is_empty() {
            self.seed_default_assets(user_id).await?;
            return sqlx::query_as::<_, Asset>(
                r#"
                SELECT id, user_id, ticker, name, asset_type, quantity, unit_value, avg_price
                FROM assets
                WHERE user_id = $1
                ORDER BY id ASC;
                "#,
            )
            .bind(user_id)
            .fetch_all(&self.db)
            .await;
        }

        Ok(assets)
    }

    pub async fn create_asset(
        &self,
        user_id: Option<i64>,
        ticker: Option<String>,
        name: String,
        asset_type: Option<String>,
        quantity: Option<f64>,
        unit_value: f64,
        avg_price: Option<f64>,
    ) -> Result<Asset, sqlx::Error> {
        let ticker = ticker.unwrap_or_else(|| name.chars().take(4).collect::<String>().to_uppercase());
        let asset_type = asset_type.unwrap_or_else(|| "Stocks".to_string());
        let quantity = quantity.unwrap_or(1.0);
        let avg_price = avg_price.unwrap_or(unit_value);

        sqlx::query_as::<_, Asset>(
            r#"
            INSERT INTO assets (user_id, ticker, name, asset_type, quantity, unit_value, avg_price)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, ticker, name, asset_type, quantity, unit_value, avg_price;
            "#,
        )
        .bind(user_id)
        .bind(ticker)
        .bind(name)
        .bind(asset_type)
        .bind(quantity)
        .bind(unit_value)
        .bind(avg_price)
        .fetch_one(&self.db)
        .await
    }

    pub async fn update_asset(
        &self,
        asset_id: i64,
        user_id: Option<i64>,
        ticker: Option<String>,
        name: Option<String>,
        asset_type: Option<String>,
        quantity: Option<f64>,
        unit_value: Option<f64>,
        avg_price: Option<f64>,
    ) -> Result<Option<Asset>, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            r#"
            UPDATE assets
            SET ticker = COALESCE($3, ticker),
                name = COALESCE($4, name),
                asset_type = COALESCE($5, asset_type),
                quantity = COALESCE($6, quantity),
                unit_value = COALESCE($7, unit_value),
                avg_price = COALESCE($8, avg_price)
            WHERE id = $1 AND ($2::BIGINT IS NULL OR user_id = $2 OR user_id IS NULL)
            RETURNING id, user_id, ticker, name, asset_type, quantity, unit_value, avg_price;
            "#,
        )
        .bind(asset_id)
        .bind(user_id)
        .bind(ticker)
        .bind(name)
        .bind(asset_type)
        .bind(quantity)
        .bind(unit_value)
        .bind(avg_price)
        .fetch_optional(&self.db)
        .await
    }

    pub async fn delete_asset(&self, asset_id: i64, user_id: Option<i64>) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM assets
            WHERE id = $1 AND ($2::BIGINT IS NULL OR user_id = $2 OR user_id IS NULL);
            "#,
        )
        .bind(asset_id)
        .bind(user_id)
        .execute(&self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn add_user(&self, username: &str, password_hash: &str) -> Result<UserRecord, sqlx::Error> {
        sqlx::query_as::<_, UserRecord>(
            r#"
            INSERT INTO users (username, password_hash)
            VALUES ($1, $2)
            RETURNING id, username, password_hash;
            "#,
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_user_by_name(&self, username: &str) -> Result<Option<UserRecord>, sqlx::Error> {
        sqlx::query_as::<_, UserRecord>(
            r#"
            SELECT id, username, password_hash
            FROM users
            WHERE username = $1;
            "#,
        )
        .bind(username)
        .fetch_optional(&self.db)
        .await
    }

    pub async fn seed_default_assets(&self, user_id: i64) -> Result<(), sqlx::Error> {
        let demo_assets = vec![
            ("AAPL", "Apple Inc.", "Stocks", 145.0, 175.50, 150.25),
            ("MSFT", "Microsoft Corp.", "Stocks", 82.5, 338.11, 310.00),
            ("BTC", "Bitcoin", "Crypto", 1.254, 39850.20, 42000.00),
            ("TSLA", "Tesla Inc.", "Stocks", 50.0, 212.45, 250.00),
            ("ETH", "Ethereum", "Crypto", 4.5, 3450.20, 2900.00),
            ("NVDA", "NVIDIA Corp", "Stocks", 30.0, 824.50, 650.00),
            ("CDI", "Tesouro Selic / CDI", "Fixed Income", 100.0, 120.00, 100.00),
        ];

        for (ticker, name, asset_type, quantity, unit_val, avg_p) in demo_assets {
            let _ = self.create_asset(
                Some(user_id),
                Some(ticker.to_string()),
                name.to_string(),
                Some(asset_type.to_string()),
                Some(quantity),
                unit_val,
                Some(avg_p),
            ).await;
        }

        // Seed demo transactions
        let demo_txs = vec![
            ("Apple Inc.", "AAPL", "Buy", 50.0, 173.44, 8672.00, "Completed"),
            ("Microsoft Corp.", "MSFT", "Dividend", 200.0, 0.68, 136.00, "Completed"),
            ("Tesla Inc.", "TSLA", "Sell", 15.0, 212.08, 3181.20, "Pending"),
            ("Alphabet Inc.", "GOOGL", "Buy", 100.0, 135.50, 13550.00, "Completed"),
        ];

        for (asset_name, ticker, tx_type, quantity, price, total_val, status) in demo_txs {
            let _ = sqlx::query(
                r#"
                INSERT INTO transactions (user_id, asset_name, ticker, tx_type, quantity, price, total_value, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
                "#,
            )
            .bind(user_id)
            .bind(asset_name)
            .bind(ticker)
            .bind(tx_type)
            .bind(quantity)
            .bind(price)
            .bind(total_val)
            .bind(status)
            .execute(&self.db)
            .await;
        }

        Ok(())
    }
}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
