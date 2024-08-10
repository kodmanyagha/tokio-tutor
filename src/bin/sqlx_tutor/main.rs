use anyhow::anyhow;
use sqlx::{mysql::MySqlPoolOptions, prelude::FromRow};

use std::any::Any;
use std::ops::Index;
use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};
use sqlx::mysql::MySqlColumn;
use sqlx::mysql::MySqlTypeInfo;
use sqlx::types::time::Date;
use sqlx::Type;
use sqlx::ValueRef;
use sqlx::{Error, MySql, MySqlPool, Pool, Row};
use time::macros::format_description;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let _a = Command::new("cmd").arg(arg!([name] "Optional name to operate on").required(false));

    let _x = command!("");

    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    let conn_str = "mysql://root:123456@127.0.0.1:3306/jr_user_micro?schema=jr_user_micro";

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(conn_str)
        .await?;
    //let pool = MySqlPool::connect(conn_str).await?;

    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    println!("row 1: {}", row.0);

    let row: (String,) = sqlx::query_as("SELECT ?")
        .bind("test".to_string())
        .fetch_one(&pool)
        .await?;
    println!("row 2: {}", row.0);

    let rows = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    for (row_index, user) in rows.iter().enumerate() {
        println!(
            "User index: {}, id: {}, email: {}",
            row_index, user.id, user.email
        );
    }

    /* Normally map() function doesn't put "index" to parameter, but if you
    use `enumerate().map()` than it will pass a tuple which contains index
    and item. */
    let indexable_map = rows
        .iter()
        .enumerate()
        .map(|(index, user)| {
            println!("x: {index}");
            (index, user.id.clone())
        })
        .collect::<Vec<(usize, String)>>();
    println!("indexable_map: {:#?}", indexable_map);

    let x = rows.iter();
    let x = x.map(|row| {
        println!("row: {}", row.id);
        row.id.clone()
    });
    let x = x.collect::<Vec<String>>();
    println!("x: {:#?}", x);

    /* `query_as()` kullanırken mutlaka gelecek olan datanın türünü belirtmen gerekiyor. */
    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ? LIMIT 1")
        .bind("32e54f7f-a4d6-4935-b65d-81589c5c0f11")
        .fetch_all(&pool)
        .await?;
    let x = query_result.get(0).ok_or(sqlx::Error::RowNotFound)?;
    println!(">>> {}", x.id);

    println!("Users: {:#?}", query_result);

    let single_result = sqlx::query("SELECT * FROM users WHERE id = ? LIMIT 1")
        .bind("32e54f7f-a4d6-4935-b65d-81589c5c0f11")
        .fetch_one(&pool)
        .await?;

    let x = single_result.column(0);
    println!("xxxxxxxxxx: {:?}", x);

    let x = single_result.columns();
    let id: String = single_result.get(0);

    let result = sqlx::query("SELECT * FROM users").fetch_all(&pool).await?;

    /* Vec türünün map fonksiyonu yok, iter(), iter().enumerate() gibi fonksiyonlardan sonra
    map() fonksiyonu açılıyor. */
    let result = result
        .iter()
        .enumerate()
        .map(|(index, row)| {
            //row.len();
            //row.get(0);
            // get() fonksiyonunun açılması için sqlx::Row traitinin import edilmesi gerekiyor.
            let x: i32 = row.get(2);
            x
        })
        .filter(|item| item.is_positive())
        .collect::<Vec<i32>>();
    println!("result: {:#?}", result);

    Ok(())
}

#[derive(FromRow, Debug)]
struct User {
    id: String,
    email: String,
}
