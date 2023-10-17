use sqlx::{sqlite::SqliteRow, FromRow, Pool, Row, Sqlite};

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub title: String,
    pub start_date: i64,
    pub end_date: i64,
}

impl FromRow<'_, SqliteRow> for Event {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("start_date")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NewEvent {
    pub title: String,
    pub start_date: i64,
    pub end_date: i64,
}

impl From<Event> for NewEvent {
    fn from(event: Event) -> Self {
        Self {
            title: event.title,
            start_date: event.start_date,
            end_date: event.end_date,
        }
    }
}

#[derive(Debug)]
pub struct Cale {
    pool: Pool<Sqlite>,
}

#[derive(Debug)]
pub enum AllowOverlap {
    Yes,
    No,
}

#[derive(Debug)]
pub struct Range {
    pub start_date: i64,
    pub end_date: i64,
}

impl Cale {
    pub fn new(pool: Pool<Sqlite>) -> Cale {
        Cale { pool }
    }

    pub async fn create(self, event: NewEvent, allow_overlap: AllowOverlap) {
        match allow_overlap {
            AllowOverlap::No => match self
                .is_overlaping(Range {
                    start_date: event.start_date,
                    end_date: event.end_date,
                })
                .await
            {
                Ok(_) => self.insert(event).await,
                Err(e) => println!("Error: {}", e),
            },
            AllowOverlap::Yes => {
                self.insert(event).await;
            }
        };
    }

    pub async fn get_events(self, range: Range) -> Result<Vec<Event>, String> {
        match sqlx::query_as::<_, Event>("SELECT * FROM events WHERE start_date BETWEEN $1 AND $2")
            .bind(range.start_date)
            .bind(range.end_date)
            .fetch_all(&self.pool)
            .await
        {
            Ok(events) => Ok(events),
            Err(e) => Err(format!("Error: {:?}", e)),
        }
    }

    pub async fn delete(self, id: u32) {
        match sqlx::query("DELETE FROM events WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => println!("Event deleted"),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub async fn update(self, event: Event, allow_overlap: AllowOverlap) {
        let event = match sqlx::query_as::<_, Event>("SELECT * from events WHERE id = $1")
            .bind(event.id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(event) => event,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        match allow_overlap {
            AllowOverlap::No => match self
                .is_overlaping(Range {
                    start_date: event.start_date,
                    end_date: event.end_date,
                })
                .await
            {
                Ok(_) => self.patch(event).await,
                Err(e) => println!("Error: {}", e),
            },
            AllowOverlap::Yes => {
                self.patch(event).await;
            }
        }
    }

    pub async fn show(self, event_id: u32) -> Result<Event, String> {
        match sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
            .bind(event_id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(event) => Ok(event),
            Err(e) => {
                return Err(format!("Error: {}", e));
            }
        }
    }

    async fn is_overlaping(&self, range: Range) -> Result<bool, String> {
        let overlaping = match sqlx::query_as::<_, Event>(
            "SELECT * FROM events WHERE
            start_date <= $1 AND
            end_date >= $2 ",
        )
        .bind(range.start_date)
        .bind(range.end_date)
        .fetch_all(&self.pool)
        .await
        {
            Ok(events) => events,
            Err(e) => return Err(format!("Error: {:?}", e)),
        };

        if overlaping.is_empty() {
            return Ok(true);
        }
        Err(String::from(
            "Event overlaps with another event. Use --allow-overlap or -a to force.",
        ))
    }

    async fn insert(&self, event: NewEvent) {
        match sqlx::query("INSERT INTO events (title, start_date, end_date) VALUES ($1, $2, $3)")
            .bind(event.title)
            .bind(event.start_date)
            .bind(event.end_date)
            .execute(&self.pool)
            .await
        {
            Ok(_) => println!("Event successfully created"),
            Err(e) => println!("Error: {}", e),
        }
    }

    async fn patch(&self, event: Event) {
        match sqlx::query(
            "UPDATE events SET title = $1, start_date = $2, end_date = $3 WHERE id = $4",
        )
        .bind(event.title)
        .bind(event.start_date)
        .bind(event.end_date)
        .bind(event.id)
        .execute(&self.pool)
        .await
        {
            Ok(_) => println!("Event updated"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
