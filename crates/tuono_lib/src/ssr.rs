use crate::mode::{Mode, GLOBAL_MODE};
use ssr_rs::{Ssr, SsrError};
use std::cell::RefCell;
use std::fs::read_to_string;
use std::path::PathBuf;

/// For the server side rendering we need to split the implementation between dev and prod.
/// This completely remove the multi-thread optimization on dev but allow the dev server to
/// update the SSR result without reloading the whole server.
pub struct Js;

#[cfg(target_os = "windows")]
const PROD_BUNDLE_PATH: &str = ".\\out\\server\\prod-server.js";
#[cfg(target_os = "windows")]
const DEV_BUNDLE_PATH: &str = ".\\.tuono\\server\\dev-server.js";

#[cfg(not(target_os = "windows"))]
const PROD_BUNDLE_PATH: &str = "./out/server/prod-server.js";
#[cfg(not(target_os = "windows"))]
const DEV_BUNDLE_PATH: &str = "./.tuono/server/dev-server.js";

impl Js {
    pub fn render_to_string(payload: Option<&str>) -> Result<String, SsrError> {
        let mode = GLOBAL_MODE.get().expect("Failed to get GLOBAL_MODE");

        if *mode == Mode::Dev {
            DevJs::render_to_string(payload)
        } else {
            ProdJs::SSR.with(|ssr| ssr.borrow_mut().render_to_string(payload))
        }
    }
}

struct ProdJs;

impl ProdJs {
    thread_local! {
        pub static SSR: RefCell<Ssr<'static, 'static>> = RefCell::new(
            Ssr::from(
                read_to_string(PathBuf::from(PROD_BUNDLE_PATH)).expect("Server bundle not found"), ""
            ).unwrap()
        )
    }
}

struct DevJs;

impl DevJs {
    pub fn render_to_string(params: Option<&str>) -> Result<String, SsrError> {
        let bundle_path = read_to_string(PathBuf::from(DEV_BUNDLE_PATH));

        if let Ok(source) = bundle_path {
            let ssr = Ssr::from(source, "");
            if let Ok(mut ssr) = ssr {
                ssr.render_to_string(params)
            } else {
                let fallback_html = read_to_string(PathBuf::from("./.tuono/index.html")).unwrap();
                Ok(fallback_html.replace("[SERVER_PAYLOAD]", params.unwrap_or("")))
            }
        } else {
            let fallback_html = read_to_string(PathBuf::from("./.tuono/index.html")).unwrap();

            Ok(fallback_html)
        }
    }
}
