use std::{
    fs, 
    error::Error, str::FromStr
};

use server::tables::*;

use sqlx::postgres as pg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = pg::PgConnectOptions::from_str(
        &fs::read_to_string("postgres_url.txt").unwrap()
    ).unwrap();

    let pool = pg::PgPool::connect_with(options).await?;

    let mut user = User::get_one(&pool, 1).await?;

    println!("{:?}", user);

    user.push_id(&pool, "clique_ids", 3).await?;
    user.push_id(&pool, "posessed_file_ids", 5).await?;

    println!("{:?}", User::get_one(&pool, user.id).await?);

    Ok(())
}
