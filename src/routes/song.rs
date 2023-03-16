use crate::request::Request;
use crate::response::Response;

pub struct Song {}

pub async fn info<'a, T>(req: &'a Request<'a>, res: &'a Response<'a, T>) {}

pub async fn urls<'a, T>(req: &'a Request<'a>, res: &'a Response<'a, T>) {}

pub async fn url<'a, T>(req: &'a Request<'a>, res: &'a Response<'a, T>) {}
