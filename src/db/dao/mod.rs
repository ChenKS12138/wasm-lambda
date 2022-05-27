pub struct Dao {
    pub pool: sqlx::MySqlPool,
}

impl Dao {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}
