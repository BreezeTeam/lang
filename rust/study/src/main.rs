
use datafusion::error::Result;
use datafusion::prelude::*;

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results
#[tokio::main]
async fn main() -> Result<()> {
    // create local execution context
    let ctx = SessionContext::new();

    // register csv file with the execution context
    ctx.register_csv(
        "task",
        r"D../../src/tasks.csv",
        CsvReadOptions::new(),
    ).await?;

    // execute the query
    let df = ctx
        .sql("SELECT * FROM task ")
        .await?;

    // print the results
    df.show().await?;

    Ok(())
}