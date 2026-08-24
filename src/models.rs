use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Asset {
    pub id: i64,
    pub user_id: Option<i64>,
    pub ticker: Option<String>,
    pub name: String,
    pub asset_type: Option<String>,
    pub quantity: Option<f64>,
    pub unit_value: f64,
    pub avg_price: Option<f64>,
}

impl Asset {
    pub fn get_ticker(&self) -> String {
        self.ticker
            .clone()
            .unwrap_or_else(|| self.name.chars().take(4).collect::<String>().to_uppercase())
    }

    pub fn get_asset_type(&self) -> String {
        self.asset_type.clone().unwrap_or_else(|| "Stocks".to_string())
    }

    pub fn get_quantity(&self) -> f64 {
        self.quantity.unwrap_or(1.0)
    }

    pub fn get_avg_price(&self) -> f64 {
        self.avg_price.unwrap_or(self.unit_value)
    }

    pub fn total_value(&self) -> f64 {
        self.get_quantity() * self.unit_value
    }

    pub fn total_cost(&self) -> f64 {
        self.get_quantity() * self.get_avg_price()
    }

    pub fn profit_loss(&self) -> f64 {
        self.total_value() - self.total_cost()
    }

    pub fn profit_loss_pct(&self) -> f64 {
        let cost = self.total_cost();
        if cost <= 0.0 {
            0.0
        } else {
            (self.profit_loss() / cost) * 100.0
        }
    }

    pub fn is_profit(&self) -> bool {
        self.profit_loss() >= 0.0
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct TransactionRecord {
    pub id: i64,
    pub user_id: Option<i64>,
    pub asset_name: String,
    pub ticker: String,
    pub tx_type: String, // "Buy", "Sell", "Dividend"
    pub quantity: f64,
    pub price: f64,
    pub total_value: f64,
    pub status: String, // "Completed", "Pending"
    pub created_at: Option<chrono_placeholder::DateTimeStr>,
}

pub mod chrono_placeholder {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type, Default)]
    #[sqlx(transparent)]
    pub struct DateTimeStr(pub String);
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PortfolioSummary {
    pub total_balance: f64,
    pub total_invested: f64,
    pub total_profit_loss: f64,
    pub total_profit_loss_pct: f64,
    pub stocks_value: f64,
    pub crypto_value: f64,
    pub fixed_income_value: f64,
    pub stocks_pct: f64,
    pub crypto_pct: f64,
    pub fixed_income_pct: f64,
    pub assets_count: usize,
}

impl PortfolioSummary {
    pub fn from_assets(assets: &[Asset]) -> Self {
        let mut total_balance = 0.0;
        let mut total_invested = 0.0;
        let mut stocks_value = 0.0;
        let mut crypto_value = 0.0;
        let mut fixed_income_value = 0.0;

        for asset in assets {
            let val = asset.total_value();
            let cost = asset.total_cost();
            total_balance += val;
            total_invested += cost;

            match asset.get_asset_type().to_lowercase().as_str() {
                "crypto" | "cripto" => crypto_value += val,
                "fixed income" | "renda fixa" | "cash" => fixed_income_value += val,
                _ => stocks_value += val,
            }
        }

        let total_profit_loss = total_balance - total_invested;
        let total_profit_loss_pct = if total_invested > 0.0 {
            (total_profit_loss / total_invested) * 100.0
        } else {
            0.0
        };

        let (stocks_pct, crypto_pct, fixed_income_pct) = if total_balance > 0.0 {
            (
                (stocks_value / total_balance) * 100.0,
                (crypto_value / total_balance) * 100.0,
                (fixed_income_value / total_balance) * 100.0,
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        Self {
            total_balance,
            total_invested,
            total_profit_loss,
            total_profit_loss_pct,
            stocks_value,
            crypto_value,
            fixed_income_value,
            stocks_pct,
            crypto_pct,
            fixed_income_pct,
            assets_count: assets.len(),
        }
    }
}
