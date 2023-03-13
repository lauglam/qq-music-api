use std::collections::HashMap;

pub struct QQMusic {
    cookie: Option<HashMap<String, String>>,
}

impl QQMusic {
    pub fn new() -> QQMusic {
        QQMusic { cookie: None }
    }

    pub fn uin(&self) -> Option<String> {
        match self.cookie {
            None => None,
            Some(ref cookie) => Some(cookie["uni"].clone())
        }
    }

    pub fn cookie(&self) -> &Option<HashMap<String, String>> {
        &self.cookie
    }

    pub fn cookie_string(&self) -> Option<String> {
        match self.cookie {
            None => None,
            Some(ref cookie) => {
                let mut res = Vec::new();
                for (key, value) in cookie {
                    res.push(format!("{}={}", key, value));
                }
                Some(res.join(";"))
            }
        }
    }

    pub fn set_cookie(&mut self, cookie_str: &str) {
        if self.cookie.is_some() {
            return;
        }

        let new = cookie_str
            .split(";")
            .map(|s| s.trim().split_at(s.find("=").unwrap()))
            .map(|(key, value)| (String::from(key), String::from(value)))
            .collect();

        self.cookie.replace(new);
    }

    // pub fn api(&self, path:&str, query: HashMap<String,String>) -> Result<>
}
