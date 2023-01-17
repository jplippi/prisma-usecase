#[path = "../generated/prisma.rs"]
mod prisma;

use prisma::{user, product, category, Role};
use dotenv::dotenv;
use rand;
use faker_rand::en_us::names::FullName;
use faker_rand::lorem::Word;
use faker_rand::util::CapitalizeFirstLetter;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let db = prisma::new_client().await?;

    for i in 1..1001 {
        let role = if i == 1 {
            Role::Admin
        } else {
            Role::User
        };

        let created = db.user()
            .create(
                format!("{}_{}{}@mailinator.com", rand::random::<Word>().to_string(), rand::random::<Word>().to_string(), i),
                "tester".to_string(),
                rand::random::<FullName>().to_string(),
                vec![
                    user::id::set(i),
                    user::role::set(role),
                ]
            )
            .exec()
            .await?;

        println!("Created user: {:?}", created);
    }

    for i in 1..301 {

        let created = db.category()
            .create(
                rand::random::<CapitalizeFirstLetter<Word>>().to_string(),
                vec![
                    category::id::set(i),
                ]
            )
            .exec()
            .await?;

        println!("Created category: {:?}", created);
    }

    for i in 1..5001 {
        let user_id = Some((i / 6) + 1 as i32);

        let category_id = rand::thread_rng().gen_range(1..301) as i32;

        let created = db.product()
            .create(
                format!("{} {}", rand::random::<CapitalizeFirstLetter<Word>>().to_string(), rand::random::<CapitalizeFirstLetter<Word>>().to_string()),
                vec![
                    product::id::set(i),
                    product::owner_id::set(user_id),
                    product::categories::connect(vec![category::id::equals(category_id)]),
                ]
            )
            .exec()
            .await?;

        println!("Created product: {:?}", created);
    }

    Ok(())
}
