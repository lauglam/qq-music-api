use std::collections::HashMap;

pub struct Request<'a> {
    query: HashMap<String, String>,
    cookies: &'a Option<HashMap<String, String>>,
}

impl<'a> Request<'a> {
    pub fn new(
        query: HashMap<String, String>,
        cookies: &'a Option<HashMap<String, String>>,
    ) -> Request {
        Request {
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
}
