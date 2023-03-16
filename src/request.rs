use awc::Client;
use std::collections::HashMap;
use awc::error::{JsonPayloadError, SendRequestError};
use awc::http::Method as AWCMethod;
use derive_more::Display;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub enum Method {
    GET,
    PUT,
    POST,
    PATCH,
    DELETE,
}

impl Into<AWCMethod> for Method {
    fn into(self) -> AWCMethod {
        match self {
            Method::GET => AWCMethod::GET,
            Method::PUT => AWCMethod::PUT,
            Method::POST => AWCMethod::POST,
            Method::PATCH => AWCMethod::PATCH,
            Method::DELETE => AWCMethod::DELETE,
        }
    }
}

pub struct Request<'a> {
    client: Client,
    query: HashMap<String, String>,
    cookies: &'a Option<HashMap<String, String>>,
}

impl<'a> Request<'a> {
    pub fn new(
        query: HashMap<String, String>,
        cookies: &'a Option<HashMap<String, String>>,
    ) -> Request {
        Request {
            client: Client::default(),
            query,
            cookies,
        }
    }

    pub fn query(&self) -> &HashMap<String, String> {
        &self.query
    }

    pub fn cookies(&self) -> &Option<HashMap<String, String>> {
        &self.cookies
    }

    pub async fn send_json<A, T>(
        &self,
        url: &str,
        method: Method,
        value: &A,
    ) -> RequestResult<T>
        where A: Serialize,
              T: DeserializeOwned,
    {
        Ok(
            self.client
                .request(method.into(), url)
                .query(&self.query)
                .unwrap()
                .send_json(value)
                .await?
                .json::<T>()
                .await?
        )
    }
}

pub type RequestResult<A> = Result<A, RequestError>;

#[derive(Debug, Display)]
pub enum RequestError {
    SendRequestError(SendRequestError),
    JsonPayloadError(JsonPayloadError),
}

impl std::error::Error for RequestError {}

impl From<SendRequestError> for RequestError {
    fn from(value: SendRequestError) -> Self {
        RequestError::SendRequestError(value)
    }
}

impl From<JsonPayloadError> for RequestError {
    fn from(value: JsonPayloadError) -> Self {
        RequestError::JsonPayloadError(value)
    }
}
