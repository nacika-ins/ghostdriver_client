
use rustc_serialize::Decoder;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use rustc_serialize::base64::FromBase64;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use cookie::Cookie as CookiePair;
use hyper::Client;

#[derive(RustcEncodable, Debug)]
struct URLObject {
    url: String,
}

#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
struct DeleteSessionObject {
    sessionId: String,
    status: i64,
    value: HashMap<String, String>,
}

#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
struct ScreenShotObject {
    sessionId: String,
    status: i64,
    value: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct CookieJSONObject {
    name: String,
    value: String,
    path: Option<String>,
    domain: Option<String>,
    secure: Option<bool>,
    httpOnly: Option<bool>,
    expiry: Option<i64>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct ValueCookieJSONObject {
    sessionId: String,
    status: i64,
    value: Vec<CookieJSONObject>,
}

#[derive(RustcEncodable, Debug)]
struct ScriptObject {
    script: String,
    args: Vec<String>,
}

#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
struct PhantomJsCreateSessionProxyObject {
    proxyType: String,
}

#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
struct PhantomJsCreateSessionValueObject {
    acceptSslCerts: bool,
    applicationCacheEnabled: bool,
    browserConnectionEnabled: bool,
    browserName: String,
    cssSelectorsEnabled: bool,
    databaseEnabled: bool,
    driverName: String,
    driverVersion: String,
    handlesAlerts: bool,
    javascriptEnabled: bool,
    locationContextEnabled: bool,
    nativeEvents: bool,
    platform: String,
    proxy: PhantomJsCreateSessionProxyObject,
    rotatable: bool,
    takesScreenshot: bool,
    version: String,
    webStorageEnabled: bool,
}

#[derive(RustcDecodable, Debug)]
#[allow(non_snake_case)]
struct PhantomJsCreateSessionObject {
    sessionId: String,
    status: i64,
    value: PhantomJsCreateSessionValueObject,
}

pub struct PhantomJSSession {
    host: String,
    port: String,
    session_id: String,
}

impl PhantomJSSession {
    pub fn jump_to_url<'a>(&'a mut self, url: String) -> bool {
        let uj = URLObject { url: url.clone() };
        let ujstr: String = json::encode(&uj).unwrap();
        let client = Client::new();
        let mut res = client.post(&format!("http://{}:{}/session/{}/url",
                                           self.host,
                                           self.port,
                                           self.session_id))
                            .body(&ujstr)
                            .send()
                            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        true
    }

    pub fn set_cookies<'a>(&'a mut self, cookies: Vec<CookiePair>) -> bool {
        for cookie in cookies {
            let expiry = {
                match cookie.expires.clone() {
                    Some(tm) => Some(tm.to_timespec().sec),
                    None => None,
                }
            };

            let c = CookieJSONObject {
                name: cookie.name.clone(),
                value: cookie.value.clone(),
                path: cookie.path.clone(),
                domain: cookie.domain.clone(),
                secure: Some(cookie.secure.clone()),
                httpOnly: Some(cookie.httponly.clone()),
                expiry: expiry.clone(),
            };
            let mut cj: HashMap<String, CookieJSONObject> = HashMap::new();
            cj.insert("cookie".to_owned(), c);
            let cjstr: String = json::encode(&cj).unwrap();

            let client = Client::new();
            let mut res = client.post(&format!("http://{}:{}/session/{}/cookie",
                                               self.host,
                                               self.port,
                                               self.session_id))
                                .body(&cjstr)
                                .send()
                                .unwrap();
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
        }
        true
    }

    pub fn capture_screenshot<'a>(&'a mut self, filepath: String) -> bool {
        let client = Client::new();
        let mut res = client.get(&format!("http://{}:{}/session/{}/screenshot",
                                          self.host,
                                          self.port,
                                          self.session_id))
                            .send()
                            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let decoded: Result<ScreenShotObject, DecoderError> = json::decode(&body);
        let decoded = decoded.unwrap();
        let base64 = decoded.value.as_bytes().from_base64().unwrap();
        let mut f = File::create(filepath).unwrap();
        let _ = f.write_all(&base64);
        true
    }

    pub fn get_cookies<'a>(&'a mut self) -> Vec<CookieJSONObject> {
        let client = Client::new();
        let mut res = client.get(&format!("http://{}:{}/session/{}/cookie",
                                          self.host,
                                          self.port,
                                          self.session_id))
                            .send()
                            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let decoded: Result<ValueCookieJSONObject, DecoderError> = json::decode(&body);
        let decoded = decoded.unwrap();
        decoded.value
    }

    pub fn execute_script<'a>(&'a mut self, script: String) -> String {
        let jsstr = ScriptObject {
            script: script.clone(),
            args: vec![],
        };
        let jsstr: String = json::encode(&jsstr).unwrap();
        let client = Client::new();
        let mut res = client.post(&format!("http://{}:{}/session/{}/execute",
                                           self.host,
                                           self.port,
                                           self.session_id))
                            .body(&jsstr)
                            .send()
                            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let decoded: Result<ScreenShotObject, DecoderError> = json::decode(&body);
        let decoded = decoded.unwrap();
        decoded.value.to_string()
    }
}

impl Drop for PhantomJSSession {
    fn drop(&mut self) -> () {
        let client = Client::new();
        let mut res = client.delete(&format!("http://{}:{}/session/{}",
                                             self.host,
                                             self.port,
                                             self.session_id))
                            .send()
                            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let decoded: Result<DeleteSessionObject, DecoderError> = json::decode(&body);
        let _ = decoded.unwrap();
    }
}

pub fn get_session(host: String, port: String, useragent: String) -> PhantomJSSession {

    let body = format!(r#"{{"desiredCapabilities":{{"platform":"ANY","browserName":"phantomjs","version":"", "phantomjs.page.settings.userAgent": "{}"}}}}"#, useragent);
    let client = Client::new();
    let mut res = client.post(&format!("http://{}:{}/session", host, port))
                        .body(&body)
                        .send()
                        .unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{}", body);
    let decoded: Result<PhantomJsCreateSessionObject, DecoderError> = json::decode(&body);
    let json = decoded.unwrap();

    PhantomJSSession {
        host: host,
        port: port,
        session_id: json.sessionId.clone(),
    }
}
