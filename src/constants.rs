extern crate serde_json;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;

//pub const SERVER_LISTEN: &str = "0.0.0.0:5001";
pub const SERVER_LISTEN: &str = "127.0.0.1:5001";

pub const CACHE_POOL_MAX_OPEN: u32 = 10;
pub const CACHE_POOL_MIN_IDLE: u32 = 2;
pub const CACHE_POOL_TIMEOUT_SECONDS: u64 = 5;
pub const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub const ANAGRAM_REDIS_PATH: &str = "unix:///var/run/redis/redis-server.sock?db=1";

lazy_static! {
    pub static ref ANAGRAM_MAPPING: HashMap<String, u64> = {
        println!("[+] [Anagram] Loading anagram configuration file");
        let file = File::open("letter_map.json").expect("Could not open letter_map.json");
        let reader = BufReader::new(file);
        return serde_json::from_reader(reader).expect("Malformed letter_map.json");
    };
}
pub const ANAGRAM_MAX_LENGTH: usize = 50;

