use shared::db::db_connector::DbConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize DB connector using env variable names
    let db = DbConnector::new(
        "AGENT_DB_USER",
        "AGENT_DB_PASS",
        "AGENT_DB_IP",
        "AGENT_DB_PORT",
        "AGENT_DB_NAME",
    ).await?;

    // execute query using library method
    let rows = db.execute_query("SELECT 1").await?;

    // print result
    for row in rows {
        println!("Result: {}", row[0]);
    }

    Ok(())
}