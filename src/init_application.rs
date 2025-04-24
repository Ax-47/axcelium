use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHasher;
use chrono::Utc;
use scylla::client::session::Session;
use scylla::value::CqlTimestamp;
use std::sync::Arc;
use uuid::Uuid;
fn hash_password(password: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    password_hash
}

pub async fn create_organization(session: Arc<Session>) {
    let organization_id = Uuid::new_v4();
    let name = "Axcelium";
    let slug = "axcelium";
    let contact_email = "support@axcelium.io";
    let is_active = true;
    let to_insert = CqlTimestamp(Utc::now().timestamp_millis());
    let created_at = to_insert;
    let updated_at = created_at;
    let query = "
    SELECT organization_id FROM axcelium.organizations
    WHERE name = ? ALLOW FILTERING;
";

    let result= session
        .query_unpaged(query, (name,))
        .await
        .unwrap()
        .into_rows_result()
        .unwrap()
        .first_row::<(Uuid,)>();
    if result.is_ok(){
        return;
    }
    let query = "
        INSERT INTO axcelium.organizations (
            organization_id, 
            name, 
            slug, 
            contact_email, 
            is_active, 
            created_at, 
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?);
    ";
    session
        .query_unpaged(
            query,
            (
                organization_id,
                name,
                slug,
                contact_email,
                is_active,
                created_at,
                updated_at,
            ),
        )
        .await
        .unwrap();

    create_application(session, organization_id).await;
}

async fn create_application(session: Arc<Session>, organization_id: Uuid) {
    let application_id = Uuid::new_v4();
    let name = "Axcelium Core";
    let description = "The core SSO platform of Axcelium.";
    let client_id = Uuid::new_v4();
    let client_secret = Uuid::new_v4();
    let hashed_client_secret = hash_password(client_secret.to_string());
    let to_insert = CqlTimestamp(Utc::now().timestamp_millis());
    let created_at = to_insert;
    let updated_at = created_at;
    let query = "
        INSERT INTO axcelium.applications (
            organization_id, 
            application_id, 
            name, 
            description, 
            client_id, 
            client_secret, 
            created_at, 
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?);
    ";

    session
        .query_unpaged(
            query,
            (
                organization_id,
                application_id,
                name,
                description,
                client_id,
                hashed_client_secret,
                created_at,
                updated_at,
            ),
        )
        .await
        .unwrap();
    println!("Organization created with ID: {}",organization_id);
    println!("Application created with ID: {}", application_id);
    println!("Client ID: {}", client_id);
    println!("Client Secret: {}", client_secret);
}
