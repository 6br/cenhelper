extern crate serde_yaml;
use std::error::Error;
use std::fmt;
use regex::Regex;
// use std::r#try;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub path: String, // Requires no prefix
    pub start: u64,
    pub stop: u64,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}:{}-{}", self.path, self.start, self.stop)
    }
}

impl Region {
    
    pub fn interval(&self) -> u64 {
        if self.inverted() {
            return self.start - self.stop
        } else {
            return self.stop - self.start
        }
    }
    pub fn inverted(&self) -> bool {
        self.start > self.stop
    }

    // It is used on converting dna-sequence region to bed-style region.
    pub fn start_minus(&mut self) {
        self.start = self.start - 1;
    }

    #[no_mangle]
    pub extern fn new_with_prefix(path: String, chr_prefix: &str) -> Result<Self, Box<dyn Error>> {
        let re = Regex::new(r"^(.+):(\d+)-?(\d*)$").unwrap();
        let caps = re.captures(&path).ok_or("Parse Error")?;
        let mut path_str = caps.get(1).ok_or("Parse Path Error")?.as_str();
        let path_string: String;
        if chr_prefix.len() == 0 {
            if path_str.starts_with("chr") {
                path_str = &path_str[3..];
            }
            path_string = path_str.to_string();
        } else {
            if path_str.starts_with(chr_prefix) {
                path_str = &path_str[chr_prefix.len()..]; // .replace("chr", "");
            }
            if path_str.len() < chr_prefix.len() {
                path_string = format!("{}{}", chr_prefix, path_str);
            } else {
                path_string = path_str.to_string()
            }
        }
        let start = caps.get(2).ok_or("Parse Start Position Error")?;
        let stop = caps.get(3).ok_or("Parse Stop Position Error")?;
        let start_str: &str = start.as_str().as_ref();
        let stop_str: &str = stop.as_str().as_ref();
        let start_u64: u64 = start_str.parse::<u64>().map_err(|e| "Parse Int Error, ".to_string() + &e.to_string())?;
        let stop_u64: u64 = stop_str.parse::<u64>().map_err(|e| "Parse Int Error, ".to_string() + &e.to_string())?;
        Ok(Region{path: path_string, start: start_u64, stop: stop_u64})
    }

    pub fn new(path: String) -> Result<Self, Box<dyn Error>> {
        let re = Regex::new(r"^(.+):(\d+)-?(\d*)$").unwrap();
        let caps = re.captures(&path).ok_or("Parse Error")?;
        let path = caps.get(1).ok_or("Parse Path Error")?;
        let start = caps.get(2).ok_or("Parse Start Position Error")?;
        let stop = caps.get(3).ok_or("Parse Stop Position Error")?;
        let start_str: &str = start.as_str().as_ref();
        let stop_str: &str = stop.as_str().as_ref();
        let start_u64: u64 = start_str.parse::<u64>().map_err(|e| "Parse Int Error, ".to_string() + &e.to_string())?;
        let stop_u64: u64 = stop_str.parse::<u64>().map_err(|e| "Parse Int Error, ".to_string() + &e.to_string())?;
        Ok(Region{path: path.as_str().to_string(), start: start_u64, stop: stop_u64})
    }

    pub fn uuid(self: &Region) -> String {
        return format!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region_format(path: &str) -> String {
        format!("{}", Region::new(path.to_string()).unwrap())
    }

    #[test]
    fn region_works() {
        assert_eq!(Region::new("".to_string()).ok(), None);
        assert_eq!(Region::new(":10-20".to_string()).ok(), None);
        assert_eq!(Region::new("chr1:12000-12001".to_string()).ok(), Some(Region{path: "chr1".to_string(), start: 12000, stop: 12001}));
        assert_eq!(Region::new("chr1:1200943-1201000".to_string()).ok(), Some(Region{path: "chr1".to_string(), start: 1200943, stop: 1201000}));
    }

    #[test]
    fn region_format_works() {
        let a = "chr1:12000-12001";
        assert_eq!(region_format(a), a);
        let b = "10:120-120001";
        assert_eq!(region_format(b), b);
    }
}