use axum::{http::{Request, StatusCode}, middleware::Next, headers::{Header, Authorization, authorization::Bearer}, response::Response};


#[derive(Clone, Debug)]
pub struct AuthState {
    pub authenticated: bool
}

pub async fn auth_user<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    let x = req.headers().get("Authorization");

    match x {
        Some(auth) => {
            let valid = validate_token(auth.to_str().unwrap());
            req.extensions_mut().insert(AuthState{authenticated: valid});
        },

        None => {
            req.extensions_mut().insert(AuthState{authenticated: false});
        }
    }

    Ok(next.run(req).await)
}

fn validate_token(token: &str) -> bool {
    match token {
        "koka123" => true,
        _ => false
    }
}