use sqlx::{Pool, Postgres};

pub async fn connect_postgres(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = Pool::<Postgres>::connect(url).await?;
    println!("Conectado a PostgreSQL");
    Ok(pool)
}