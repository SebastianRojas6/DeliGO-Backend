use sqlx::PgPool;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    // debemos cambiarlos configurando la carga de variables de entorno, chamba de braulio daaaaaa
    let database_url = "postgres://user:password@localhost/dbname";
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}
