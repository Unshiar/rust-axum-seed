mod user;

use crate::api::user::*;
use utoipa::openapi::{InfoBuilder, Server};
use utoipa::OpenApi;

pub struct ApiDoc;

impl ApiDoc {
    pub fn openapi() -> utoipa::openapi::OpenApi {
        let mut main_api = utoipa::openapi::OpenApi::default();

        main_api.info = InfoBuilder::new()
            .title(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .build();

        main_api.servers = Some(vec![Server::new("http://localhost:8080")]);

        main_api.merge(UserApi::openapi());

        main_api
    }
}
