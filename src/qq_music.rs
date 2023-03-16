use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use crate::request::Request;
use crate::response::{Response, ResponseResult};

pub struct QQMusic {
    cookies: RefCell<Option<HashMap<String, String>>>,
}

impl QQMusic {
    pub fn new() -> QQMusic {
        QQMusic { cookies: RefCell::new(None) }
    }

    pub fn uin(&self) -> Option<String> {
        match self.cookies.borrow().as_ref() {
            None => None,
            Some(cookies) => Some(cookies["uni"].clone())
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

    pub fn api<T>(&self, path: &str, mut query: HashMap<String, String>) {
        query.insert("ownCookie".to_string(), "1".to_string());
        let request = Request::new(query, &self.cookies.borrow());

        let response: Response<T> = Response::new(
            &|res| self.result_handle(res),
            &|url| self.redirect_handle(url),
            &|key, value| self.cookie_handle(key, value),
        );


    }

    fn result_handle<T>(&self, result: ResponseResult<T>) {}

    fn redirect_handle(&self, url: &str) {}

    fn cookie_handle(&self, key: &str, value: &str) {
        let key = key.to_string();
        let value = key.to_string();
        let mut cookies = self.cookies.borrow_mut();
        match cookies.as_mut() {
            None => {
                let new = HashMap::from([(key, value)]);
                cookies.replace(new);
            }
            Some(cookies) => {
                cookies.insert(key, value);
            }
        }
    }
}
