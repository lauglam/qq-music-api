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

use crate::request::Request;
use crate::response::Response;

pub async fn route<'a, T>(path: &str, req: &'a Request<'a>, res: &'a Response<'a, T>) {
    match path {
        "/song" => song::info(req, res).await,
        "/song/urls" => song::urls(req, res).await,
        "/song/url" => song::url(req, res).await,
        _ => unreachable!(),
    }
}
