use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, near_bindgen};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner: Option<AccountId>,
    pub files: UnorderedMap<String, Vec<u8>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: None,
            files: UnorderedMap::new(b"files".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        if request.path == "/" {
            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: self.files.get(&"/index.html".to_owned()).expect("404 Not found").into(),
            }
        } else if let Some(file) = self.files.get(&request.path) {
            Web4Response::Body {
                content_type: get_content_type(&request.path).to_owned(),
                body: file.into(),
            }
        } else if let Some(file) = self.files.get(&(request.path.clone() + ".html")) {
            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: file.into(),
            }
        } else {
            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: format!("<h1>404 Not found</h1><pre>{:#?}</pre>", request).into_bytes().into(),
            }
        }
    }

    pub fn clear(&mut self) {
        self.check_owner();
        self.files.clear();
    }

    pub fn upload_file(&mut self, name: String, data: Vec<u8>) {
        self.check_owner();
        self.files.insert(&name, &data);
    }

    #[private]
    pub fn check_owner(&mut self) {
        if self.owner.is_none() {
            self.owner = Some(near_sdk::env::predecessor_account_id());
        }
        let caller = near_sdk::env::predecessor_account_id();
        assert_eq!(&caller, self.owner.as_ref().unwrap(), "Only owner can clear the files");
    }
}

fn get_content_type(file_name: &str) -> &'static str {
    match file_name.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub path: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub query: std::collections::HashMap<String, Vec<String>>,
    pub preloads: Option<std::collections::HashMap<String, Web4Response>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum Web4Response {
    Body {
        #[serde(rename = "contentType")]
        content_type: String,
        body: near_sdk::json_types::Base64VecU8,
    },
    BodyUrl {
        #[serde(rename = "bodyUrl")]
        body_url: String,
    },
    PreloadUrls {
        #[serde(rename = "preloadUrls")]
        preload_urls: Vec<String>,
    },
}
