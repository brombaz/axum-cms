use std::env;

use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, crypto::verify, DecodingKey, decode, Validation, Algorithm};

use crate::models::{auth::Claims, error::{Error, Result}};

use super::JWT_DURATION_IN_SECONDS;

pub fn hash_password() {

}

pub fn verify_hash() {

}

pub fn create_jwt(email: String) -> Result<String>{
	let jwt_secret = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());
	let mut now = Utc::now();
	let iat = (now.timestamp() as usize); // Issued at
	let expires_in = Duration::seconds(JWT_DURATION_IN_SECONDS);
	now+= expires_in;
	let exp = now.timestamp() as usize; // Expires at

	let claim = Claims {
		exp,
		iat,
		email
	};
	let token = encode(&Header::default(), &claim, &jwt_secret).map_err(|_| Error::InternalServerError);
	token
}

pub fn is_jwt_valid(token: &str) -> Result<bool>{
	let secret = env::var("JWT_SECRET").unwrap();
	let key = &DecodingKey::from_secret(secret.as_bytes());
	
	let is_decoded = decode::<Claims>(token, key, &Validation::new(Algorithm::HS256)).map_err(|e| {
		match e.kind() {
			jsonwebtoken::errors::ErrorKind::ExpiredSignature => { Error::AuthFailCookieExpired },
			_ => { Error::InvalidJwt }
		}
	})?;

	Ok(true)
}