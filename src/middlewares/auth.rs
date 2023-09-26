use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;
use crate::{models::error::{Result, Error}, utils::auth::is_jwt_valid, middlewares::AUTHORIZATION_HEADER};

use super::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
	cookies: Cookies,
	req: Request<B>,
	next: Next<B>
) -> Result<Response> {
	println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
	let auth_cookie = cookies.get(AUTHORIZATION_HEADER).map(|c| c.value().to_string());

	// TODO: Real auth-token parsing & validation.
	let auth_token = auth_cookie.ok_or(Error::AuthFailNoAuthTokenCookie)?;
	let cookie_info = auth_token.split_whitespace().collect::<Vec<&str>>();

	if cookie_info[0] != "Bearer" {
		return Err(Error::AuthFailNoAuthTokenCookie)
	}
	
	if let Some(&jwt) = cookie_info.get(1) {
		is_jwt_valid(jwt)?;
	} else {
		return Err(Error::AuthFailNoAuthTokenCookie)
	}

	Ok(next.run(req).await)
}

