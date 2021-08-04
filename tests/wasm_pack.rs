// wasm-pack test --node --features bundled

use rusqlite::{params, Connection};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn node_test() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )
    .unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .unwrap();

    let people: Vec<_> = person_iter.collect();
    assert_eq!(
        format!("{:?}", people),
        r#"[Ok(Person { id: 1, name: "Steven", data: None })]"#
    );
}
