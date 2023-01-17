#[path = "./generated/prisma.rs"]
mod prisma;

use dotenv::dotenv;
use crate::prisma::user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db = prisma::new_client()
        .await
        .expect("Failed to create Prisma client");

    let user = db
        .user()
        .find_unique(user::id::equals(1))
        .with(user::products::fetch(vec![]))
        .exec()
        .await?;
    
    println!("{:?}", user);

    let users = db
        .user()
        .find_many(vec![])
        .with(user::products::fetch(vec![]))
        .exec()
        .await?;
    
    println!("{:?}", users[0].products);
    
    return Ok(());
}
