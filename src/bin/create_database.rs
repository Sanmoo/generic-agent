use rusqlite::Connection;

fn main() {
    let connection = Connection::open("db.sqlite").unwrap();

    let query = "
        CREATE TABLE commands (
            id INTEGER primary key AUTOINCREMENT,
            created_at DATETIME not null,
            started_at DATETIME,
            finished_at DATETIME,
            command TEXT not null,
            status INTEGER,
            stdout TEXT,
            stderr TEXT
        );
    ";

    connection.execute( query, []).unwrap();
}