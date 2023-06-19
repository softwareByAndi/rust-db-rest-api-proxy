use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use tokio_postgres::{NoTls, Error};
use serde_json::json;


fn print_type_of<T>(_: &T) {
    print!("{}", std::any::type_name::<T>());
}

enum SqlValue {
    Text(String),
    Integer(i32),
    Null
}


#[get("/")]
async fn index() -> impl Responder {
    println!("hello world!");


    // Connect to the database
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=test dbname=test password='hello world'", NoTls)
            .await
            .unwrap();
    println!("connected to database");



    // Spawn a task to process the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        } else {
            println!("connection made");
        }
    });
    println!("spawned task");



    // Perform database operations
    let rows: Vec<tokio_postgres::Row> = match client
        .query("SELECT * FROM test.test_table", &[])
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("query error: {}", e);
            Vec::<tokio_postgres::Row>::new()
        }
    };
    



    let json: serde_json::Value = rows
        .iter()
        .map(|row: &tokio_postgres::Row| {
            let columns = row.columns();
            let values: Vec<serde_json::Value> = columns
                .iter()
                .map(|column| {
                    // println!("column name: {}", column.name());
                    // println!("column type: {}", column.type_().name());

                    // FIXME : pull this out into a function
                    let value: SqlValue = match column.type_().name() {
                        "int4" => SqlValue::Integer(row.get::<_, i32>(column.name())),
                        "varchar" => SqlValue::Text(row.get::<_, String>(column.name())),
                        "null" => SqlValue::Null,
                        _ => {
                            let error_message = format!("ERROR : unknown type {} : {}", column.name(), column.type_().name());
                            println!("{}", &error_message);
                            SqlValue::Text(error_message)
                        }
                    };
                    match value {
                        SqlValue::Integer(value) => json!(value),
                        SqlValue::Text(value) => json!(value),
                        SqlValue::Null => json!(null),
                    }
                })
                .collect();
            json!(values)
        })
        .collect();

    println!("Values: {:?}", json);
    HttpResponse::Ok().json(json)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("null") // for local host testing
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec!["Content-Type"])
                    .max_age(3600)
            )
            .service(index)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

