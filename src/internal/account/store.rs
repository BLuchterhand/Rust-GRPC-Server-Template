use tokio_postgres::Client;


#[derive(Debug)]
pub struct DBClient {
    pub client: tokio_postgres::Client,
}

impl DBClient {
    pub fn new(client: Client) -> Self {
        DBClient {client}
    }

    pub async fn login(&self, email: String) -> Result<tokio_postgres::Row, tokio_postgres::Error> {
        let query = format!("
            SELECT id, password 
            FROM accounts 
            where email = $1");
        let account_info = self.client.query_one(&query, &[&email]).await?;
        Ok(account_info)
    }

    pub async fn get_account_by_email(&self, email: &String) -> Result<tokio_postgres::Row, tokio_postgres::Error> {
        let query = format!("
            SELECT EXISTS(SELECT 1 FROM accounts where email = $1)");
        let account = self.client.query_one(&query, &[&email]).await?;
        Ok(account)
    }

    pub async fn signup(&self, email: String, password: String) -> Result<tokio_postgres::Row, tokio_postgres::Error> {
        let statement = self.client.prepare("
            INSERT INTO accounts (email, password) 
            VALUES ($1, $2)
            RETURNING id").await?;
        let result = self.client.query_one(&statement, &[&email, &password]).await?;
        Ok(result)
    }
}