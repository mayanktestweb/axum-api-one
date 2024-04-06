use std::fmt::format;

use axum::{Json, extract::{Path, Query, FromRequest}, http::{HeaderMap, StatusCode, Request}, Extension, response::{Response, IntoResponse}, body::HttpBody, BoxError, async_trait, RequestExt};
use axum::extract::TypedHeader;
use axum::headers::{UserAgent, ContentType};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use validator::Validate;

use crate::middlewares::auth::AuthState;

use super::SharedData;
//use serde_json::{json, Value};


pub async fn hello_world() ->  String {
    "Hello, World! from handler xx !".to_string()
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TestJson {
    name: String,
    age: i32
}

pub async fn extract_json(Json(body): Json<TestJson>) -> Json<TestJson> {
    Json(body)
}


pub async fn handle_path(Path(x): Path<TestJson>) -> String {
    format!("name is {} and age is {}", x.name, x.age)
}

pub async fn handle_query(Query(x): Query<TestJson>) -> Json<TestJson> {
    Json::from(TestJson {
        name: x.name,
        age: x.age
    })
}


pub async fn extract_user_agent_header(TypedHeader(head): TypedHeader<UserAgent>) -> String {
    head.to_string()
}

pub async fn extract_content_type_header(TypedHeader(head): TypedHeader<ContentType>) -> String {
    head.to_string()
}

pub async fn extract_custom_header(header: HeaderMap) -> String {
    header.get("x-message").unwrap().to_str().unwrap().to_owned()
}



pub async fn extract_shared_data(Extension(sd): Extension<SharedData>) -> String {
    sd.message
}


#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}



pub async fn custom_201_response() -> Response {
    (
        StatusCode::IM_USED,
        "It's used buddy"
    ).into_response()
}


#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct QueryParams {
    value: i32
}

pub async fn get_json(Query(query): Query<QueryParams>) -> Result<Json<Value>, Response> {
    
    
    match query.value {
        0 => Err((StatusCode::IM_A_TEAPOT, "Teapot error! he he haha!!!").into_response()),
        _ => Ok(Json::from(
            json!({
                "name": "Fakkad",
                "age": 30,
                "game": "cricket"
            })
        ))
    }
}



#[derive(Debug, Deserialize, Validate)]
pub struct UserInput {
    #[validate(email(message = "Invalid Email!"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password should have atleast 8 chars"))]
    pub password: String
}

// define extractor for above struct
#[async_trait]
impl<S, B> FromRequest<S, B> for UserInput 
where   B: HttpBody + Send + 'static,
        B::Data: Send,
        B::Error: Into<BoxError>,
{
    type Rejection = Response;

    async fn from_request(req: Request<B> ,state: &S) ->  Result<Self, Self::Rejection> {
        let Json(user_input) = req.extract::<Json<UserInput>, _>()
            .await
            .map_err(|err| err.into_response())?;

        match user_input.validate() {
            Ok(_) => Ok(user_input),
            Err(err) => Err((StatusCode::BAD_REQUEST, format!("{}", err)).into_response())
        }
    }
}



pub async fn json_by_extractor(user_input: UserInput) {
    dbg!(user_input);
}



// play with authentication middleware


pub async fn auth_route<B>(req: Request<B>) -> Result<String, Response> 
where B: HttpBody + Send + 'static,
B::Data: Send,
B::Error: Into<BoxError>,
{
    let authorized = req.extensions().get::<AuthState>().unwrap().clone();
    
    let Json(x) = &req.extract::<Json<TestJson>, _>()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, "provide name and age!")
        .into_response())?;


    Ok(format!("name: {} age: {} auth is {}", x.name, x.age, authorized.authenticated))
}