use gluesql::{execute, Payload, SledStorage, Store};
use nom_sql::parse_query;
use std::fmt::Debug;

pub trait Helper<T: 'static + Debug> {
    fn get_storage(&self) -> &dyn Store<T>;

    fn run(&self, sql: &str) -> Result<Payload<T>, ()> {
        let parsed = parse_query(sql).unwrap();
        let storage = self.get_storage();

        println!("[Run] {}", parsed);
        execute(storage, &parsed)
    }

    fn run_and_print(&self, sql: &str) {
        let result = self.run(sql);

        match result.unwrap() {
            Payload::Select(rows) => println!("[Ok ]\n{:#?}\n", rows),
            Payload::Insert(row) => println!("[Ok ]\n{:#?}\n", row),
            Payload::Delete(num) => println!("[Ok ] {} rows deleted.\n", num),
            Payload::Update(num) => println!("[Ok ] {} rows updated.\n", num),
            Payload::Create => println!("[Ok ] :)\n"),
        };
    }

    fn test_rows(&self, sql: &str, count: usize) {
        let result = self.run(sql);

        match result.unwrap() {
            Payload::Select(rows) => assert_eq!(count, rows.len()),
            Payload::Delete(num) => assert_eq!(count, num),
            Payload::Update(num) => assert_eq!(count, num),
            _ => panic!("compare is only for Select, Delete and Update"),
        };
    }

    fn test_columns(&self, sql: &str, count: usize) {
        let result = self.run(sql);

        match result.unwrap() {
            Payload::Select(rows) => {
                assert_eq!(count, rows.into_iter().nth(0).unwrap().items.len())
            }
            _ => assert!(false),
        };
    }
}

pub struct SledHelper {
    storage: Box<SledStorage>,
}

impl SledHelper {
    pub fn new(path: &str) -> Self {
        let storage = Box::new(SledStorage::new(path.to_string()));

        SledHelper { storage }
    }
}

impl Helper<u64> for SledHelper {
    fn get_storage(&self) -> &dyn Store<u64> {
        self.storage.as_ref()
    }
}
