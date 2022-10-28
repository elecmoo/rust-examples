use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+$").expect("init regex err");
        if let Some(r) = re.captures(s) {
            r.get(0)
                .map_or(0, |s| -> Self { s.as_str().parse().unwrap_or(0) })
        } else {
            0
        }
    }
}
