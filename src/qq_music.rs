use crate::request::Request;
use crate::response::{Response, ResponseError, ResponseResult};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use crate::routes;

pub struct QQMusic {
    cookies: RefCell<Option<HashMap<String, String>>>,
}

impl QQMusic {
    pub fn new() -> QQMusic {
        QQMusic {
            cookies: RefCell::new(None),
        }
    }

    pub fn uin(&self) -> Option<String> {
        match self.cookies.borrow().as_ref() {
            None => None,
            Some(cookies) => Some(cookies["uni"].clone()),
        }
    }

    pub fn cookies(&self) -> Ref<Option<HashMap<String, String>>> {
        self.cookies.borrow()
    }

    pub fn cookie_string(&self) -> Option<String> {
        match self.cookies.borrow().as_ref() {
            None => None,
            Some(cookies) => {
                let mut res = Vec::new();
                for (key, value) in cookies {
                    res.push(format!("{}={}", key, value));
                }
                Some(res.join(";"))
            }
        }
    }

    pub fn set_cookies(&self, cookie_str: &str) {
        if self.cookies.borrow().is_some() {
            return;
        }
        let new = cookie_str
            .split(";")
            .map(|s| s.trim().split_at(s.find("=").unwrap()))
            .map(|(key, value)| (String::from(key), String::from(value)))
            .collect();

        self.cookies.borrow_mut().replace(new);
    }

    pub async fn api<T: DeserializeOwned>(&self, path: &str, mut query: HashMap<String, String>) -> ResponseResult<T> {
        let mut res = Err(ResponseError::Pending);

        let mut result_handle = |r| res = r;
        let redirect_handle = |url| unimplemented!();
        let cookie_handle = |key: &str, value: &str| {
            let mut cookies = self.cookies.borrow_mut();
            match cookies.as_mut() {
                None => {
                    let new = HashMap::from([(key.into(), value.into())]);
                    cookies.replace(new);
                }
                Some(cookies) => {
                    cookies.insert(key.into(), value.into());
                }
            }
        };

        query.insert("ownCookie".into(), "1".into());
        let cookies = self.cookies.borrow();

        let request = Request::new(query, &cookies);
        let response = Response::new(
            &mut result_handle,
            &redirect_handle,
            &cookie_handle,
        );

        routes::route(path, request, response).await;

        res
    }
}
