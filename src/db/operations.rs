use sqlx::PgPool;
use crate::db::models::Contact;

pub async fn create_contact(pool: &PgPool, contact: &Contact) -> sqlx::Result<()> {
   let query = "INSERT INTO Contact(contact_id, name, phone_number, email, address, birthday) VALUES ($1, $2, $3, $4, $5, $6)";
   sqlx::query(query)
       .bind(&contact.contact_id)
       .bind(&contact.name)
       .bind(&contact.phone_number)
       .bind(&contact.email)
       .bind(&contact.address)
       .bind(&contact.birthday)
       .execute(pool)
       .await?;
   Ok(())
}

pub async fn bulk_create_contacts(pool: &PgPool, contacts: Vec<Contact>) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;

    for contact in contacts {
        let _ = sqlx::query!(r#"INSERT INTO Contact(contact_id, name, phone_number, email, address, birthday) VALUES ($1, $2, $3, $4, $5, $6)"#,
            &contact.contact_id,
            &contact.name,
            contact.phone_number.as_deref(),
            contact.email.as_deref(),
            contact.address.as_deref(),
            contact.birthday.as_deref())
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(())

}

pub async fn read_contact(pool: &PgPool, contact_id: i32) -> sqlx::Result<Contact> {
    let query = "SELECT * FROM Contact WHERE contact_id = $1";
    let contact = sqlx::query_as::<_, Contact>(query)
        .bind(contact_id)
        .fetch_one(pool)
        .await?;
    Ok(contact)
}

// update contact
// delete contact
// list all contacts
pub async fn list_all_contacts(pool: &PgPool) -> sqlx::Result<Vec<Contact>> {
    let query = "SELECT * FROM Contact";
    let contacts: Vec<Contact> = sqlx::query_as::<_, Contact>(query).fetch_all(pool).await?;
    Ok(contacts)
}
