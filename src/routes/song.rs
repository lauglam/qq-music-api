use serde::de::DeserializeOwned;
use serde_json::json;
use crate::request::{Method, Request, RequestResult};
use crate::response::Response;

pub struct Song {}

pub(crate) async fn info<'a, T: DeserializeOwned>(req: Request<'a>, mut res: Response<'a, T>) {
    const URL: &str = "http://u.y.qq.com/cgi-bin/musicu.fcg";
    let song_mid = req.query().get(r#"songmid"#).unwrap();
    let data = json!({
        "xsrfCookieName": "XSRF-TOKEN",
        "withCredentials": true,
        "data": {
            "songinfo": {
                "method": "get_song_detail_yqq",
                "module": "music.pf_song_detail_svr",
                "param": {
                    "song_mid": song_mid
                }
            }
        }
    });

    let result: RequestResult<T> = req.send_json(URL, Method::GET, &data).await;
    res.result(match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.into()),
    })
}

pub(crate) async fn urls<'a, T: DeserializeOwned>(req: Request<'a>, mut res: Response<'a, T>) {}

pub(crate) async fn url<'a, T: DeserializeOwned>(req: Request<'a>, mut res: Response<'a, T>) {}
