pub use crate::routes::{home, login, profile, register, search, upload};

pub use crate::api::api_upload;

pub use crate::utils::{language, translations};

pub use crate::components::common::load_defaults;

pub use tera::Tera;
pub use tide::Server;
pub use tide::{Request, Response, StatusCode};
pub use tide_tera::prelude::*;
