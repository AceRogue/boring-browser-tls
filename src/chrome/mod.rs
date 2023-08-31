use crate::{BrowserSetting, BrowserVersion};

mod v110;

pub fn get_browser_setting(version: BrowserVersion) -> BrowserSetting {
    match version {
        BrowserVersion::ChromeV110 => v110::get_browser_settings(),
    }
}
