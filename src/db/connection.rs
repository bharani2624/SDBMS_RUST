use arangors::Connection;
use dotenv::dotenv;
use std::env;
pub async fn connect_db()-> arangors::database::Database {
    dotenv().ok();
    let url=env::var("ARANGO_URL").unwrap_or("http://localhost:8529".to_string());
    let username=env::var("ARANGO_USER").unwrap_or("root".to_string());
    let password=env::var("ARANGO_PASS").unwrap_or("2624".to_string());
    let db_name=env::var("ARANGO_DB").unwrap_or("students_db".to_string());
    // let collection_name=env::var("ARANGO_COLLECTION").unwrap_or("student".to_string());
    let conn=Connection::establish_jwt(&url.as_str(),&username,&password)
        .await
        .expect("Not Connected To ArangoDB");
    let database=conn.db(&db_name)
        .await
        .expect("DB not found");
    // let collection=database.collection(&collection_name)
    // .await
    // .expect("Collection Not Found ");
    println!("Successfully Connected To The DB:{} with Collection {}",db_name,collection_name)
    database

}