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

    pub fn handle(&self, msg: &String) -> bool {
        let json: JsonPayload = json::decode(msg).unwrap();
        self.filter.matches(&json.date)
    }
}


#[cfg(test)]
mod tests {
    use super::PayloadHandler;

    #[test]
    fn should_pass_on_correct_date() {
        let handler = PayloadHandler::new().unwrap();
        assert_eq!(handler.handle(&"{\"date\":\"2016-10-10\"}".to_string()), true);
    }

    #[test]
    fn should_fail_on_wrong_date() {
        let handler = PayloadHandler::new().unwrap();
        assert_eq!(handler.handle(&"{\"date\":\"aaaa-bb-cc\"}".to_string()), false);
    }

    #[test]
    #[should_panic]
    fn should_panic_on_bad_json() {
        let handler = PayloadHandler::new().unwrap();
        assert_eq!(handler.handle(&"{date:aaaa-bb-cc}".to_string()), false);
    }
}
