mod db;

use db::connection::connect;
use db::models::Contact;
use db::operations::{create_contact, read_contact, list_all_contacts};

async fn run_migrations(pool: &sqlx::PgPool) -> Result <(), Box<dyn std::error::Error>>{
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

async fn add_dummy_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let contacts = vec![
        Contact {
            contact_id: 1,
            name: "John Doe".to_string(),
            phone_number: Some("123-456-7890".to_string()),
            email: Some("johndoe@example.com".to_string()),
            address: Some("123 Elm St, Springfield".to_string()),
            birthday: Some("1990-01-01".to_string())
        },
        Contact {
            contact_id: 2,
            name: "Jane Smith".to_string(),
            phone_number: Some("234-567-8901".to_string()),
            email: Some("janesmith@example.com".to_string()),
            address: Some("456 Maple Ave, Rivertown".to_string()),
            birthday: Some("1985-05-05".to_string())
        },
        Contact {
            contact_id: 3,
            name: "Robert Johnson".to_string(),
            phone_number: Some("345-678-9012".to_string()),
            email: Some("robertj@example.com".to_string()),
            address: Some("789 Oak Lane, Hillside".to_string()),
            birthday: Some("1980-10-10".to_string())
        },
    ];

    for contact in contacts {
        create_contact(pool, &contact).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result <(), Box<dyn std::error::Error>>{

    let pool = connect().await?;
    run_migrations(&pool).await?;

    let contacts =  list_all_contacts(&pool).await?; // = read_contact(&pool, 0).await?;
    for contact in contacts {
        println!("{:?}", contact);
    }
    Ok(())


}
