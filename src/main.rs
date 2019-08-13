#[macro_use]
extern crate lazy_static;

use core::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::ops::Index;
use std::thread;
use csv::StringRecord;
use rayon::prelude::*;

fn main() {
    let mut rdr = csv::Reader::from_reader(BufReader::new(File::open("test.csv").unwrap()));

    rdr.records().par_bridge()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .flat_map(|record| to_bl(record))
        .for_each(|x| println!("{:?}, {}", thread::current().id(), x));
}

struct BL {
    asin: String,
    from: String,
    to: String,
    first_seen: u32,
}

impl Display for BL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.asin, self.from, self.to, self.first_seen)
    }
}

fn to_bl(record: StringRecord) -> Vec<BL> {
    asins(record.index(0))
        .into_iter()
        .map(|asin| BL {
            asin,
            from: record.index(0).to_string(),
            to: record.index(1).to_string(),
            first_seen: record.index(2).parse().unwrap_or(0),
        })
        .collect::<Vec<BL>>()
}

fn asins(url: &str) -> Vec<String> {
    lazy_static! {
        static ref ASIN_PATTERN: regex::Regex = regex::Regex::new(r"^B\d{2}\w{7}|\d{9}(?:X|\d)$").unwrap();
    }

    url.split("/").map(|s| s.to_string()).filter(|x| ASIN_PATTERN.is_match(x)).collect::<Vec<String>>()
}