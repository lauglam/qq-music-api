use serde::de::DeserializeOwned;
use crate::request::Request;
use crate::response::Response;

pub(crate) async fn info<'a, T: DeserializeOwned>(req: Request<'a>, mut res: Response<'a, T>) {}
