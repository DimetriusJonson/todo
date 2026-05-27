use std::env;

use app::run;
use dotenvy::dotenv;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let env_file_name = format!(".env.{}", environment);
    println!(
        "environment={}, env_file_name={}",
        environment, env_file_name
    );

    dotenv().ok();
    dotenvy::from_filename_override(env_file_name).ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let bind_addr = env::var("APP_BIND_ADDR").expect("APP_BIND_ADDR must be set.");

    match run(&db_url, &bind_addr).await {
        Ok(_) => println!("Done!"),
        Err(err) => println!("Error: {:?}", err),
    }
}
