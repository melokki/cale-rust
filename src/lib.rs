use sqlx::{sqlite::SqliteQueryResult, Error, FromRow, Pool, Sqlite};

#[derive(Debug, Clone, FromRow)]
pub struct Event {
    pub id: u32,
    pub title: String,
    pub start_date: i64,
    pub end_date: i64,
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

    pub async fn create(
        self,
        event: NewEvent,
        allow_overlap: AllowOverlap,
    ) -> Result<SqliteQueryResult, Error> {
        match allow_overlap {
            AllowOverlap::No => {
                match self
                    .is_overlaping(Range {
                        start_date: event.start_date,
                        end_date: event.end_date,
                    })
                    .await
                {
                    Ok(is_overlaping) => {
                        if is_overlaping {
                            return Err(sqlx::Error::RowNotFound);
                        }

                        return self.insert(event).await;
                    }
                    Err(e) => Err(e),
                }
            }

            AllowOverlap::Yes => return self.insert(event).await,
        }
    }

    pub async fn get_events(self, range: Range) -> Result<Vec<Event>, Error> {
        sqlx::query_as::<_, Event>("SELECT * FROM events WHERE start_date BETWEEN $1 AND $2")
            .bind(range.start_date)
            .bind(range.end_date)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn delete(self, id: u32) -> Result<SqliteQueryResult, Error> {
        sqlx::query("DELETE FROM events WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
    }

    pub async fn update(
        self,
        event: Event,
        allow_overlap: AllowOverlap,
    ) -> Result<SqliteQueryResult, Error> {
        match sqlx::query_as::<_, Event>("SELECT * from events WHERE id = $1")
            .bind(event.id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => match allow_overlap {
                AllowOverlap::No => {
                    match self
                        .is_overlaping(Range {
                            start_date: event.start_date,
                            end_date: event.end_date,
                        })
                        .await
                    {
                        Ok(is_overlaping) => {
                            if is_overlaping {
                                return Err(sqlx::Error::RowNotFound);
                            }

                            return self.patch(event).await;
                        }
                        Err(e) => Err(e),
                    }
                }
                AllowOverlap::Yes => return self.patch(event).await,
            },
            Err(e) => Err(e),
        }
    }

    pub async fn show(self, event_id: u32) -> Result<Event, Error> {
        sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
            .bind(event_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn is_overlaping(&self, range: Range) -> Result<bool, Error> {
        if let Ok(events) = sqlx::query_as::<_, Event>(
            "SELECT * FROM events WHERE
            start_date <= $1 AND
            end_date >= $2 ",
        )
        .bind(range.start_date)
        .bind(range.end_date)
        .fetch_all(&self.pool)
        .await
        {
            if events.is_empty() {
                return Ok(false);
            }

            return Ok(true);
        }

        Err(sqlx::Error::RowNotFound)
    }

    async fn insert(&self, event: NewEvent) -> Result<SqliteQueryResult, Error> {
        sqlx::query("INSERT INTO events (title, start_date, end_date) VALUES ($1, $2, $3)")
            .bind(event.title)
            .bind(event.start_date)
            .bind(event.end_date)
            .execute(&self.pool)
            .await
    }

    async fn patch(&self, event: Event) -> Result<SqliteQueryResult, Error> {
        sqlx::query("UPDATE events SET title = $1, start_date = $2, end_date = $3 WHERE id = $4")
            .bind(event.title)
            .bind(event.start_date)
            .bind(event.end_date)
            .bind(event.id)
            .execute(&self.pool)
            .await
    }
}
