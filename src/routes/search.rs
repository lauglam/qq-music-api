use serde::de::DeserializeOwned;
use serde_json::json;
use crate::request::{Method, Request};
use crate::response::Response;

const DEFAULT_PAGE_NO: &str = "1";
const DEFAULT_NUM_PER_PAGE: &str = "20";
const DEFAULT_TYPE: &str = "0"; // 0：单曲，2：歌单，7：歌词，8：专辑，9：歌手，12：mv

pub(crate) async fn info<'a, T: DeserializeOwned>(req: Request<'a>, mut res: Response<'a, T>) {
    let query = req.query();
    let default_page_no = DEFAULT_PAGE_NO.into();
    let default_num_per_page = DEFAULT_NUM_PER_PAGE.into();
    let default_type = DEFAULT_TYPE.into();

    let key = query.get("key").unwrap();
    let page_no = query.get("page_no").or(Some(&default_page_no)).unwrap().parse::<u32>().unwrap();
    let num_per_page = query.get("num_per_page").or(Some(&default_num_per_page)).unwrap().parse::<u32>().unwrap();
    let t = query.get("type").or(Some(&default_type)).unwrap();

    let data;
    let url;
    match t.as_str() {
        // song_list
        "2" => {
            data = json!({
                "query": key,
                "page_no": page_no - 1,
                "num_per_page": num_per_page,
            });
            url = format!(
                "{}&page_no={}&num_per_page={}&query={}",
                "https://c.y.qq.com/soso/fcgi-bin/client_music_search_songlist?remoteplace=txt.yqq.playlist",
                page_no - 1,
                num_per_page,
                key,
            );
        }
        _ => {
            url = "http://c.y.qq.com/soso/fcgi-bin/client_search_cp".into();
            data = json!({
                "format": "json", // 返回json格式
                    "n": num_per_page, // 一页显示多少条信息
                    "p": page_no, // 第几页
                    "w": key, // 搜索关键词
                    "cr": 1, // 不知道这个参数什么意思，但是加上这个参数你会对搜索结果更满意的
                    "g_tk": 5381,
                    "t": t,
            });
        }
    }

    let result = req.send_json(&url,Method::GET,&data).await;
    res.result(match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.into()),
    })
}
