use std::env;

use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use dotenv::dotenv;

use cale::{AllowOverlap, Cale, Event, NewEvent, Range};
use sqlx::SqlitePool;

/// Simple program to create events into a database
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Command to execute
    #[command(subcommand)]
    cmd: Commands,

    /// Force creating the event
    #[arg(short, long)]
    allow_overlap: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add {
        title: String,
        start_date: String,
        end_date: String,
    },
    Delete {
        id: u32,
    },
    List {
        start_date: String,
        end_date: String,
    },
    Show {
        id: u32,
    },
    Update {
        id: u32,
        title: String,
        start_date: String,
        end_date: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            println!("Cannot get DATABASE_URL: {}", e);
            return;
        }
    };
    let pool = match SqlitePool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            println!("Cannot connect to database: {}", e);
            return;
        }
    };

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => println!(""),
        Err(e) => {
            println!("Cannot run migrations: {}", e);
        }
    };

    let mut cale = Cale::new(pool.clone());

    match args.cmd {
        Commands::Add {
            title,
            start_date,
            end_date,
        } => {
            let start_date_timestamp =
                match NaiveDateTime::parse_from_str(&start_date.trim(), "%Y-%m-%d %H:%M") {
                    Ok(date) => date.and_utc().timestamp(),
                    Err(e) => {
                        println!("Cannot parse start_date: {}", e);
                        return;
                    }
                };

            let end_date_timestamp =
                match NaiveDateTime::parse_from_str(&end_date.trim(), "%Y-%m-%d %H:%M") {
                    Ok(date) => date.and_utc().timestamp(),
                    Err(e) => {
                        println!("Cannot parse end_date: {}", e);
                        return;
                    }
                };
            match args.allow_overlap {
                true => {
                    cale.create(
                        NewEvent {
                            title,
                            start_date: start_date_timestamp,
                            end_date: end_date_timestamp,
                        },
                        AllowOverlap::Yes,
                    )
                    .await
                }
                false => {
                    cale.create(
                        NewEvent {
                            title,
                            start_date: start_date_timestamp,
                            end_date: end_date_timestamp,
                        },
                        AllowOverlap::No,
                    )
                    .await
                }
            }
        }
        Commands::Delete { id } => {
            cale.delete(id).await;
        }
        Commands::Show { id } => {
            if let Ok(event) = cale.show(id).await {
                println!("{:#?}", event);
            }
        }
        Commands::List {
            start_date,
            end_date,
        } => {
            let start_date_timestamp =
                match NaiveDateTime::parse_from_str(&start_date.trim(), "%Y-%m-%d %H:%M") {
                    Ok(date) => date.and_utc().timestamp(),
                    Err(e) => {
                        println!("Cannot parse start_date: {}", e);
                        return;
                    }
                };

            let end_date_timestamp =
                match NaiveDateTime::parse_from_str(&end_date.trim(), "%Y-%m-%d %H:%M") {
                    Ok(date) => date.and_utc().timestamp(),
                    Err(e) => {
                        println!("Cannot parse end_date: {}", e);
                        return;
                    }
                };

            if let Ok(events) = cale
                .get_events(Range {
                    start_date: start_date_timestamp,
                    end_date: end_date_timestamp,
                })
                .await
            {
                println!("{:#?}", events);
            }
        }
        Commands::Update {
            id,
            title,
            start_date,
            end_date,
        } => {
            let start_date_timestamp =
                match NaiveDateTime::parse_from_str(&start_date.trim(), "%Y-%m-%d %H:%M") {
                    Ok(date) => date.and_utc().timestamp(),
                    Err(e) => {
                        println!("Cannot parse start_date: {}", e);
                        return;
                    }
                };

            let end_date_timestamp =
                match NaiveDateTime::parse_from_str(&end_date.trim(), "%Y-%m-%d %H:%M") {
                    Ok(date) => date.and_utc().timestamp(),
                    Err(e) => {
                        println!("Cannot parse end_date: {}", e);
                        return;
                    }
                };
            match args.allow_overlap {
                true => {
                    cale.update(
                        Event {
                            id,
                            title,
                            start_date: start_date_timestamp,
                            end_date: end_date_timestamp,
                        },
                        AllowOverlap::Yes,
                    )
                    .await
                }
                false => {
                    cale.update(
                        Event {
                            id,
                            title,
                            start_date: start_date_timestamp,
                            end_date: end_date_timestamp,
                        },
                        AllowOverlap::No,
                    )
                    .await
                }
            }
        }
    }
}
