mod album;
mod comment;
mod data;
mod feedback;
mod index;
mod lyric;
mod mv;
mod new;
mod radio;
mod recommend;
mod search;
mod singer;
mod song;
mod song_list;
mod test;
mod top;
mod user;

use serde::de::DeserializeOwned;
use crate::request::Request;
use crate::response::Response;

pub(crate) async fn route<'a, T: DeserializeOwned>(path: &str, req: Request<'a>, res: Response<'a, T>) {
    match path {
        "/song" => song::info(req, res).await,
        "/song/urls" => song::urls(req, res).await,
        "/song/url" => song::url(req, res).await,

        "/search" => search::info(req, res).await,
        _ => unreachable!(),
    }
}
