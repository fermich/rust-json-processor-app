use regex::Regex;
use std::result::Result;
use rustc_serialize::json;

const DATE_REGEXP: &'static str = r"^\d{4}-\d{2}-\d{2}$";

struct RegexFilter {
    reg: Regex
}
impl RegexFilter {
    pub fn new(regexp: &str) -> Option<RegexFilter> {
        match Regex::new(regexp) {
            Result::Ok(r) => Some(RegexFilter { reg: r }),
            Result::Err(_) => None
        }
    }

    pub fn matches(&self, text: &String) -> bool {
        self.reg.is_match(text)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct JsonPayload {
    date: String
}

pub struct PayloadHandler {
    filter: RegexFilter
}
impl PayloadHandler {
    pub fn new() -> Option<PayloadHandler> {
        match RegexFilter::new(DATE_REGEXP) {
            Some(filter) => Some(PayloadHandler { filter: filter }),
            None => None
        }
    }

    pub fn handle(&self, msg: &String) {
        let json: JsonPayload = json::decode(msg).unwrap();
        println!("Matches? {}", self.filter.matches(&json.date))
    }
}
