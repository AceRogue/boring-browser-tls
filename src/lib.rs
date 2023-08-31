use std::sync::Arc;

use boring::ssl::SslConnectorBuilder;
use reqwest::header::HeaderMap;

pub mod chrome;

#[derive(Clone)]
pub struct BrowserSetting {
    pub tls_builder: Arc<dyn Fn() -> SslConnectorBuilder + Sync + Send>,
    pub default_headers: HeaderMap,
}

#[derive(Debug)]
pub enum BrowserVersion {
    ChromeV110,
}

pub fn get_browser_setting(version: BrowserVersion) -> BrowserSetting {
    match version {
        BrowserVersion::ChromeV110 => chrome::get_browser_setting(version),
    }
}
