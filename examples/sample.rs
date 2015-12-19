extern crate ghostdriver_client;
extern crate cookie;
use cookie::Cookie as CookiePair;
use std::collections::BTreeMap;

fn main() {

    let mut phantomjs_session = ghostdriver_client::get_session("localhost".to_owned(),
                                                                "8910".to_owned(),
                                                                "Mozilla/5.0 (Macintosh; Intel \
                                                                 Mac OS X 10_11_0) \
                                                                 AppleWebKit/537.36 (KHTML, like \
                                                                 Gecko) Chrome/46.0.2490.80 \
                                                                 Safari/537.36"
                                                                    .to_owned());
    phantomjs_session.jump_to_url("http://google.com".to_owned());
    phantomjs_session.execute_script("document.body.style.backgroundColor = 'red'; return '1';"
                                         .to_owned());
    phantomjs_session.capture_screenshot("foo.png".to_owned());
    phantomjs_session.set_cookies(vec![CookiePair {
                                           name: "foo".to_owned(),
                                           value: "bar".to_owned(),
                                           expires: None,
                                           max_age: None,
                                           domain: None,
                                           path: None,
                                           secure: false,
                                           httponly: false,
                                           custom: BTreeMap::new(),
                                       }]);
    println!("{:?}", phantomjs_session.get_cookies());

}
