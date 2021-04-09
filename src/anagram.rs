use actix_web::{get, web, HttpResponse};
use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::{Commands, FromRedisValue};
use r2d2::{PooledConnection};
use crate::constants::{ANAGRAM_MAPPING, ANAGRAM_MAX_LENGTH, CACHE_POOL_TIMEOUT_SECONDS};
use serde::{Serialize, Deserialize};
use std::time::{Duration};

// Redis connection pool type
type RedisPool = r2d2::Pool<RedisConnectionManager>;

#[derive(Serialize, Deserialize)]
struct AnagramsList {
    anagrams: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct AnagramError {
    message: String
}

#[get("/find/{letters}")]
pub async fn handle_find_anagrams(pool: web::Data<RedisPool>, web::Path(letters): web::Path<String>) -> HttpResponse {

    let anagram_hash_opt = anagram_hash(&letters);

    let anagram_hash_val = match anagram_hash_opt {
        Err(e) => return HttpResponse::BadRequest().json(AnagramError {
            message: e
        }),
        Ok(val) => val
    };

    let mut conn = match pool.get_timeout(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)) {
        Ok(val) => val,
        Err(_) => return HttpResponse::InternalServerError().json(AnagramError {
            message: "Could not obtain a connection from the connection pool".to_owned()
        })
    };

    let get_anagram_result = match web::block(move || get_anagrams(anagram_hash_val, &mut conn))
        .await {
            Ok(val) => val,
            Err(_) => return HttpResponse::InternalServerError().json(AnagramError {
                message: "An error occured".to_owned()
            })
    };

    HttpResponse::Ok().json(AnagramsList {
        anagrams: get_anagram_result
    })
}

fn get_anagrams(hash: u64, db: &mut PooledConnection<RedisConnectionManager>) -> Result<Vec<String>, i32> {

    let query_res = db.smembers(hash).unwrap();
    match FromRedisValue::from_redis_value(&query_res) {
        Ok(val) => Ok(val),
        Err(_) => Err(-1)
    }

}

fn anagram_hash(letters: &String) -> Result<u64, String> {

    let letters_chars = letters.chars();
    if letters_chars.to_owned().count() > ANAGRAM_MAX_LENGTH {
        return Err("Too many letters supplied".to_owned());
    }

    let mut hash_val: u64 = 1;
    for c in letters_chars {

        let chr_string: String = c.to_string();

        let prime_opt = ANAGRAM_MAPPING.get(&chr_string);
        match prime_opt {
            None => return Err("Invalid characters provided".to_owned()),
            Some(prime) => hash_val = hash_val * prime
        };

    }

    Ok(hash_val)
}

