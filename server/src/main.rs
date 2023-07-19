use tokio_postgres::{NoTls, Error};
use std::{
    thread::{self, JoinHandle},
    sync::{
        mpsc,
        Arc,
        Mutex,
    },
    fs, 
    net::{TcpListener, TcpStream}, 
    io::{BufReader, BufRead}
};

use server::tables::*;
// use server::auth::*;
use sha256::digest;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let query_string = fs::read_to_string("./query_string.sql").unwrap();
    let config = fs::read_to_string("./tokio_postgres.conf").unwrap();
    let server_address = fs::read_to_string("./server_address.txt").unwrap();

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect(&config, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query(&query_string, &[])
        .await?;

    // println!("{:#?}", &rows);


    let users: Vec<_> = rows.into_iter()
        .map(|row| User::from(row))
        .collect();

    
    println!("{:#?}", users);

    

    let listener = TcpListener::bind(&server_address).unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap()).await;
    }

    Ok(())
}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    println!("{:#?}", &buf_reader);
    println!("{:#?}", buf_reader.lines().next());

}