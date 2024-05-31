use std::fmt::Debug;

use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)]
/// 查数据返回
pub struct PageData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub total_pages: u64,
    pub page_num: u64,
}
/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParams {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

/// 数据统一返回格式
#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<u32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

/// 填入到extensions中的数据
#[derive(Debug, Clone)]
pub struct ResJsonString(pub String);

#[allow(unconditional_recursion)]
impl<T> IntoResponse for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()))
                    .body(Body::from(e.to_string()))
                    .unwrap();
            }
        };
        let res_json_string = ResJsonString(json_string.clone());
        let mut response = json_string.into_response();
        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl<T: Serialize> Res<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            msg: "success",
        }
    }

    pub fn with_err(err: &Error) -> Self {
        Self {
            code: err.code(),
            data: None,
            msg: err.msg(),
        }
    }
    
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, err: &Error) -> Self {
        Self {
            code: err.code(),
            data: Some(data),
            msg: err.msg(),
        }
    }
}