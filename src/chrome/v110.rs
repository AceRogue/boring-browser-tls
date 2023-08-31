use std::sync::Arc;

use boring::ssl::{SslConnector, SslConnectorBuilder, SslMethod, SslVersion};
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, UPGRADE_INSECURE_REQUESTS,
    USER_AGENT,
};

use crate::BrowserSetting;

pub fn get_browser_settings() -> BrowserSetting {
    BrowserSetting {
        tls_builder: Arc::new(create_boring_connector_builder),
        default_headers: default_headers(),
    }
}

fn create_boring_connector_builder() -> SslConnectorBuilder {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

    builder.set_grease_enabled(true);

    builder.enable_ocsp_stapling();

    let cipher_list = [
        "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_CBC_SHA",
    ];

    builder.set_cipher_list(&cipher_list.join(":")).unwrap();

    let sigalgs_list = [
        "ecdsa_secp256r1_sha256",
        "rsa_pss_rsae_sha256",
        "rsa_pkcs1_sha256",
        "ecdsa_secp384r1_sha384",
        "rsa_pss_rsae_sha384",
        "rsa_pkcs1_sha384",
        "rsa_pss_rsae_sha512",
        "rsa_pkcs1_sha512",
    ];

    builder.set_sigalgs_list(&sigalgs_list.join(":")).unwrap();
    builder.enable_signed_cert_timestamps();
    builder.set_alpn_protos(b"\x02h2\x08http/1.1").unwrap();
    builder
        .set_min_proto_version(Some(SslVersion::TLS1_2))
        .unwrap();
    builder
        .set_max_proto_version(Some(SslVersion::TLS1_3))
        .unwrap();
    builder
}

fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            r#""Chromium";v="110", "Not A(Brand";v="24", "Google Chrome";v="110"#,
        ),
    );
    headers
}
