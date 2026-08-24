use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get},
};
use serde::Deserialize;

use crate::{
    app::AppState,
    auth::user::User,
    error::AppError,
    models::{Asset, PortfolioSummary},
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", get(list_assets).post(create_asset).patch(update_asset))
        .route("/assets/{id}", delete(delete_asset_by_path))
        .route("/portfolio/summary", get(portfolio_summary))
}

#[tracing::instrument(skip_all)]
async fn list_assets(
    maybe_user: Option<User>,
    repository: Repository,
) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = if let Some(user) = maybe_user {
        repository.list_assets_by_user(user.id()).await?
    } else {
        repository.list_assets().await?
    };
    Ok(Json(assets))
}

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    pub ticker: Option<String>,
    pub name: String,
    pub asset_type: Option<String>,
    pub quantity: Option<f64>,
    pub unit_value: f64,
    pub avg_price: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    maybe_user: Option<User>,
    repository: Repository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let user_id = maybe_user.map(|u| u.id());
    let new_asset = repository
        .create_asset(
            user_id,
            request.ticker,
            request.name,
            request.asset_type,
            request.quantity,
            request.unit_value,
            request.avg_price,
        )
        .await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    pub id: i64,
    pub ticker: Option<String>,
    pub name: Option<String>,
    pub asset_type: Option<String>,
    pub quantity: Option<f64>,
    pub unit_value: Option<f64>,
    pub avg_price: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    maybe_user: Option<User>,
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let user_id = maybe_user.map(|u| u.id());
    match repository
        .update_asset(
            request.id,
            user_id,
            request.ticker,
            request.name,
            request.asset_type,
            request.quantity,
            request.unit_value,
            request.avg_price,
        )
        .await?
    {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::AssetDoesNotExist),
    }
}

#[tracing::instrument(skip_all)]
async fn delete_asset_by_path(
    maybe_user: Option<User>,
    repository: Repository,
    Path(id): Path<i64>,
) -> Result<Json<bool>, AppError> {
    let user_id = maybe_user.map(|u| u.id());
    let deleted = repository.delete_asset(id, user_id).await?;
    if deleted {
        Ok(Json(true))
    } else {
        Err(AppError::AssetDoesNotExist)
    }
}

#[tracing::instrument(skip_all)]
async fn portfolio_summary(
    maybe_user: Option<User>,
    repository: Repository,
) -> Result<Json<PortfolioSummary>, AppError> {
    let assets = if let Some(user) = maybe_user {
        repository.list_assets_by_user(user.id()).await?
    } else {
        repository.list_assets().await?
    };

    let summary = PortfolioSummary::from_assets(&assets);
    Ok(Json(summary))
}
