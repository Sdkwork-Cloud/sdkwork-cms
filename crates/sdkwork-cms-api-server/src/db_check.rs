#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("SDKWORK_CMS_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/sdkwork_cms".to_string());

    println!("Attempting to connect to: {}", database_url);

    let config = sdkwork_pool_config::DatabaseConfig {
        engine: sdkwork_pool_config::DatabaseEngine::Postgres,
        url: database_url,
        max_connections: 1,
        ..Default::default()
    };

    match sdkwork_pool_sqlx::create_pool_from_config(config).await {
        Ok(pool) => {
            println!("✅ Connected to database successfully!");
            
            // Check if tables exist
            let tables: Vec<(String,)> = sqlx::query_as(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name"
            )
            .fetch_all(pool.as_postgres().unwrap())
            .await?;
            
            println!("\nExisting tables:");
            for (table_name,) in &tables {
                println!("  - {}", table_name);
            }
            
            if tables.is_empty() {
                println!("  (no tables found)");
            }
        }
        Err(e) => {
            println!("❌ Failed to connect: {}", e);
            println!("\nPlease ensure:");
            println!("1. PostgreSQL is running on localhost:5432");
            println!("2. Database 'sdkwork_cms' exists");
            println!("3. User 'postgres' with password 'postgres' has access");
            println!("\nTo create the database, run:");
            println!("  CREATE DATABASE sdkwork_cms;");
        }
    }

    Ok(())
}
