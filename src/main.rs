use sqlx::{FromRow, PgPool};
use serde::{Deserialize, Serialize};
use inquire::{Select, Text};
use std::process;

#[derive(Debug, Serialize, Deserialize, FromRow)]

struct User{
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    println!("Terhubung ke database PostgreSQL\n");

    loop {
        let options = vec![
            "Tambah User",
            "Lihat User",
            "Update User",
            "Hapus User",
            "Keluar",
        ];

        let choice = Select::new("Pilih Operasi CRUD", options).prompt();

        match choice {
            Ok("Tambah User") => {
                let name = Text::new("Nama").prompt().unwrap();
                let email = Text::new("Email").prompt().unwrap();
                create_user(&pool, &name, &email).await?;
            }
            Ok("Lihat User") => {
                let users = get_users(&pool).await?;
                println!("\n=== Data Users ===");
                for user in users{
                    println!("ID: {} | Nama: {} | | Email: {}", user.id, user.name, user.email);
                }
                println!()
            }
            Ok("Update User") => {
                let id = Text::new("Masukan ID User yang mau di update:")
                .prompt()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let new_name = Text::new("Nama Baru : ").prompt().unwrap();
            update_user(&pool, id, &new_name).await?;
            }

            Ok("Hapus User") => { }
            Ok("Keluar") => {
                println!("Keluar dari program...");
                process::exit(0);
            }
            _ => println!("Pilihan tidak valid."),
        }
    }


}

async fn create_user(pool: &PgPool, name: &str, email: &str)->Result<(), sqlx::Error>{
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    .bind(name)
    .bind(email)
    .execute(pool)
    .await?;

    println!("User '{}' berhasil di tambahkan!\n", name);

    Ok(())
}

async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error>{
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id")
    .fetch_all(pool)
    .await?;
   Ok(users)
}

async fn update_user(pool: &PgPool, id: i32, new_name: &str)-> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
    .bind(new_name)
    .bind(id)
    .execute(pool)
    .await?;
println!("User dengan ID {} berhasil diupdate\n", id);
Ok(())
}