use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::constants;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    token_id: String,
    sub: String,
    domain: String,
    name: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct AuthBody {
    token: String,
}






