use anyhow::Result;
use sqlx::postgres::{PgPool, PgQueryAs};

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct Thing {
    pub id: String,
}

async fn get_all() -> Result<Vec<Thing>> {
    let database_url = "postgres://username:password@localhost:5432/my_database";
    let pool = PgPool::new(&database_url).await?;
    let table_name = "my_table";

    Ok(sqlx::query_as::<_, Thing>("SELECT * from $1")
        .bind(&table_name)
        .fetch_all(&pool)
        .await?)
}

#[tokio::main]
async fn main() {
    match get_all().await {
        Ok(things) => println!("{}", things.len()),
        Err(_) => println!("Error while getting things"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bind_table_name() -> anyhow::Result<()> {
        let things = get_all().await?;
        let expectation: Vec<Thing> = vec![];
        assert_eq!(things, expectation);
        Ok(())
    }
}
