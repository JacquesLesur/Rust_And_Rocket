#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate postgres;
extern crate dotenv;
use dotenv::dotenv;
use postgres::{Connection, TlsMode};
use rocket::response::NamedFile;
use std::io;
use std::mem;
use std::env;


fn main() {
    dotenv().ok();
    dotenv::dotenv().expect("Failed to read .env file");//check if .env file founded
    rocket::ignite().mount("/", routes![account_by_id,account,index]).launch();
}
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
#[get("/account/id/<id>")]
fn account_by_id(id: u32) -> &'static str {
    
    let query = "SELECT * FROM public.account WHERE user_id =".to_string();
    let query_and_id = query + &id.to_string();
    let res = execut_query(query_and_id);
    res
}
#[get("/account")]
fn account() -> &'static str {
    execut_query("SELECT * FROM public.account".to_string())
}
fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
fn execut_query(query: String)-> &'static str{
    let mut res = String::new();
    //get connexion string in .env file
    let conn_string = match env::var("CON_STRING") {
    Ok(val) => val,
    Err(e) => panic!("could not find {}: {}", "CON_STRING", e),
    };

    let conn = Connection::connect(conn_string, TlsMode::None).unwrap();
    for row in &conn.query(  &query, &[]).unwrap() {
        let id: i32 = row.get("user_id");
        let username: String = row.get("username");
        res = format!("id: {}, username: {}", id, username).clone();
        
    }
    let final_result: &'static str = string_to_static_str(res); //string to static str
    final_result
}
