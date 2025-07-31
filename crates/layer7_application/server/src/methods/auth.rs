use super::{HandleError, RawResponse, get_pool, handler};
// use crate::models::{auth, user};
// use chrono::{Days, Utc};
// use serde_json::Value;
// use warp::reply::{Reply, json, with_header};

pub async fn get_api_key_list(_user: String, _api_key: Option<String>) -> Box<dyn Reply> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

// async fn get_api_key_list_(user: String, api_key: Option<String>) -> RawResponse {
//     let pool = get_pool().await;
//     let user = user::get_detail(&user, pool).await.handle_error(404)?;
//     user::check_auth(&user.id, api_key, pool)
//         .await
//         .handle_error(500)?
//         .handle_error(403)?;
//     let api_keys = auth::get_api_key_list(&user.id, pool)
//         .await
//         .handle_error(500)?;
//     Ok(Box::new(json(&api_keys)))
// }

pub async fn create_api_key(_user: String, _body: Value) -> Box<dyn Reply> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

// async fn create_api_key_(user: String, body: Value) -> RawResponse {
//     let pool = get_pool().await;
//     let body = serde_json::from_value::<auth::ApiKeyCreation>(body).handle_error(400)?;

//     match auth::check_password(&user, &body.password, pool).await {
//         Ok(true) => {
//             let expire_after = body.expire_after;
//             let now = Utc::now();
//             let expire = now
//                 .checked_add_days(Days::new(expire_after))
//                 .handle_error(400)?;
//             let new_api_key = auth::create_api_key(&user, &body.name, expire, pool)
//                 .await
//                 .handle_error(500)?;
//             Ok(Box::new(with_header(
//                 new_api_key,
//                 "Content-Type",
//                 "text/plain; charset=utf-8",
//             )))
//         }
//         Ok(false) => Err((403, String::from("password incorrect"))),
//         Err(e) => Err((500, format!("{e:?}"))),
//     }
// }