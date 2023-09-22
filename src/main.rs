mod db;

use db::connection::connect;
use db::models::Contact;
use db::operations::{create_contact, read_contact, list_all_contacts, bulk_create_contacts};

async fn run_migrations(pool: &sqlx::PgPool) -> Result <(), Box<dyn std::error::Error>>{
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

async fn add_dummy_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let contacts = vec![
        Contact {
            contact_id: 4,
            name: "Alice Williams".to_string(),
            phone_number: Some("456-789-0123".to_string()),
            email: Some("alice@example.com".to_string()),
            address: Some("123 Birch Rd, Mountaintop".to_string()),
            birthday: Some("1992-02-12".to_string())
        },
        Contact {
            contact_id: 5,
            name: "David Brown".to_string(),
            phone_number: Some("567-890-1234".to_string()),
            email: Some("davidb@example.com".to_string()),
            address: Some("456 Pine Drive, Seaside".to_string()),
            birthday: Some("1988-03-03".to_string())
        },
        Contact {
            contact_id: 6,
            name: "Eva Martinez".to_string(),
            phone_number: Some("678-901-2345".to_string()),
            email: Some("eva@example.com".to_string()),
            address: Some("789 Cedar Blvd, Plainsville".to_string()),
            birthday: Some("1995-07-17".to_string())
        },
        Contact {
            contact_id: 7,
            name: "Tom Anderson".to_string(),
            phone_number: Some("789-012-3456".to_string()),
            email: Some("tom@example.com".to_string()),
            address: Some("123 Fir St, Foresttown".to_string()),
            birthday: Some("1986-12-31".to_string())
        }
    ];

    bulk_create_contacts(pool, contacts).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result <(), Box<dyn std::error::Error>>{

    let pool = connect().await?;
    run_migrations(&pool).await?;

    // Run the bulk create function
    add_dummy_data(&pool).await?;

    let contacts =  list_all_contacts(&pool).await?; // = read_contact(&pool, 0).await?;
    for contact in contacts {
        println!("{:?}", contact);
    }
    Ok(())


}
