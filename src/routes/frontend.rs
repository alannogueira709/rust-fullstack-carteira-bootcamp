use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{
    app::AppState,
    auth::user::{UnauthenticatedUser, User},
    error::AppError,
    models::{Asset, PortfolioSummary, TransactionRecord},
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout).post(logout))
        .route("/assets/create", post(create_asset_form))
        .route("/assets/update", post(update_asset_form))
        .route("/assets/delete", post(delete_asset_form))
        .route("/transactions/create", post(create_transaction_form))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page(maybe_user: Option<User>) -> Result<Response, AppError> {
    if maybe_user.is_some() {
        return Ok(Redirect::to("/").into_response());
    }
    let html = LoginPage.render()?;
    Ok(Html(html).into_response())
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err),
    };

    let token = user.auth_token()?;
    let cookie = Cookie::build(("token", token)).path("/").http_only(true);

    Ok((jar.add(cookie), Redirect::to("/")))
}

async fn logout(jar: CookieJar) -> Result<impl IntoResponse, AppError> {
    let mut cookie = Cookie::build(("token", "")).path("/").http_only(true);
    cookie.make_removal();
    Ok((jar.add(cookie), Redirect::to("/login")))
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate<'a> {
    username: &'a str,
    assets: &'a [Asset],
    summary: &'a PortfolioSummary,
    transactions: &'a [TransactionRecord],
}

async fn index(maybe_user: Option<User>, repository: Repository) -> Result<Response, AppError> {
    let user = match maybe_user {
        Some(u) => u,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    let assets = repository.list_assets_by_user(user.id()).await?;
    let summary = PortfolioSummary::from_assets(&assets);

    let transactions = sqlx::query_as::<_, TransactionRecord>(
        r#"
        SELECT id, user_id, asset_name, ticker, tx_type, quantity, price, total_value, status, NULL as created_at
        FROM transactions
        WHERE user_id = $1
        ORDER BY id DESC;
        "#,
    )
    .bind(user.id())
    .fetch_all(&repository.db)
    .await
    .unwrap_or_default();

    let template = DashboardTemplate {
        username: user.username(),
        assets: &assets,
        summary: &summary,
        transactions: &transactions,
    };

    let html = template.render()?;
    Ok(Html(html).into_response())
}

#[derive(Deserialize)]
struct CreateAssetForm {
    ticker: String,
    name: String,
    asset_type: String,
    quantity: f64,
    unit_value: f64,
    avg_price: Option<f64>,
}

async fn create_asset_form(
    user: User,
    repository: Repository,
    Form(form): Form<CreateAssetForm>,
) -> Result<Redirect, AppError> {
    let _ = repository
        .create_asset(
            Some(user.id()),
            Some(form.ticker),
            form.name,
            Some(form.asset_type),
            Some(form.quantity),
            form.unit_value,
            form.avg_price,
        )
        .await?;

    Ok(Redirect::to("/#assets"))
}

#[derive(Deserialize)]
struct UpdateAssetForm {
    id: i64,
    ticker: String,
    name: String,
    asset_type: String,
    quantity: f64,
    unit_value: f64,
    avg_price: Option<f64>,
}

async fn update_asset_form(
    user: User,
    repository: Repository,
    Form(form): Form<UpdateAssetForm>,
) -> Result<Redirect, AppError> {
    let _ = repository
        .update_asset(
            form.id,
            Some(user.id()),
            Some(form.ticker),
            Some(form.name),
            Some(form.asset_type),
            Some(form.quantity),
            Some(form.unit_value),
            form.avg_price,
        )
        .await?;

    Ok(Redirect::to("/#assets"))
}

#[derive(Deserialize)]
struct DeleteAssetForm {
    id: i64,
}

async fn delete_asset_form(
    user: User,
    repository: Repository,
    Form(form): Form<DeleteAssetForm>,
) -> Result<Redirect, AppError> {
    let _ = repository.delete_asset(form.id, Some(user.id())).await?;
    Ok(Redirect::to("/#assets"))
}

#[derive(Deserialize)]
struct CreateTransactionForm {
    ticker: String,
    asset_name: String,
    tx_type: String,
    quantity: f64,
    price: f64,
}

async fn create_transaction_form(
    user: User,
    repository: Repository,
    Form(form): Form<CreateTransactionForm>,
) -> Result<Redirect, AppError> {
    let total_value = form.quantity * form.price;
    let _ = sqlx::query(
        r#"
        INSERT INTO transactions (user_id, asset_name, ticker, tx_type, quantity, price, total_value, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'Completed');
        "#,
    )
    .bind(user.id())
    .bind(form.asset_name)
    .bind(form.ticker)
    .bind(form.tx_type)
    .bind(form.quantity)
    .bind(form.price)
    .bind(total_value)
    .execute(&repository.db)
    .await?;

    Ok(Redirect::to("/#transactions"))
}
