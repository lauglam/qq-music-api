use derive_more::{Display, From};

pub struct Response<'a, T> {
    result: &'a mut dyn FnMut(ResponseResult<T>),
    redirect: &'a dyn Fn(&'a str),
    cookie: &'a dyn Fn(&'a str, &'a str),
}

impl<'a, T> Response<'a, T> {
    pub fn new<S, R, C>(
        result: &'a mut S,
        redirect: &'a R,
        cookie: &'a C,
    ) -> Response<'a, T>
        where S: FnMut(ResponseResult<T>),
              R: Fn(&'a str),
              C: Fn(&'a str, &'a str),
    {
        Response {
            result,
            redirect,
            cookie,
        }
    }

    pub fn result(&mut self, res: ResponseResult<T>) {
        (self.result)(res);
    }

    pub fn redirect(&self, url: &'a str) {
        (self.redirect)(url);
    }

    pub fn cookies(&self, key: &'a str, value: &'a str) {
        (self.cookie)(key, value);
    }
}

pub type ResponseResult<T> = Result<T, ResponseError>;

#[derive(Debug, Display, From)]
pub enum ResponseError {
    #[display(fmt = "pending")]
    Pending,
}

impl std::error::Error for ResponseError {}
