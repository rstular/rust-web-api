#[macro_use]
extern crate lazy_static;

extern crate r2d2_redis;
use actix_web::{web, App, HttpServer};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::time::Duration;

mod anagram;
mod constants;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let anagram_manager = RedisConnectionManager::new(constants::ANAGRAM_REDIS_PATH).unwrap();
    let anagram_pool = r2d2::Pool::builder()
        .max_size(constants::CACHE_POOL_MAX_OPEN)
        .min_idle(Some(constants::CACHE_POOL_MIN_IDLE))
        .max_lifetime(Some(Duration::from_secs(
            constants::CACHE_POOL_EXPIRE_SECONDS,
        )))
        .build(anagram_manager)
        .expect("[Anagram] Could not build Redis connection pool");

    println!("[+] [Anagram] Built connection pool");

    return HttpServer::new(move || {
        App::new().service(
            web::scope("/anagram")
                .data(anagram_pool.clone())
                .service(anagram::handle_find_anagrams),
        )
    })
    .bind(constants::SERVER_LISTEN)
    .expect("[!] [Anagram] Could not bind to target port")
    .run()
    .await;
}
