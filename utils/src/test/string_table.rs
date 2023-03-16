use std::time::Instant;

use crate::string_table::StringTable;

#[test]
fn should_handle_insertions() {
    let mut table = StringTable::new();
    table.insert("Test");
    table.insert("Test1");
    table.insert("Test2");
    table.insert("Test3");
    table.insert("Test");
    let hash = table.hash("Test");
    assert!(table.contains(hash))
}

#[test]
fn should_handle_retrievals() {
    let mut table = StringTable::new();
    table.insert("Test");
    table.insert("Test2");
    table.insert("Test3");
    table.insert("Test4");
    let r = table.get(fxhash::hash32("Test")).unwrap();
    let slice = &r[0..1];
    assert_eq!(r, "Test");
    assert_eq!(slice, "T");
}

#[test]
fn should_handle_removals() {
    let mut table = StringTable::new();
    table.insert("Test");
    table.insert("Test2");
    table.insert("Test3");
    table.insert("Test4");
    table.remove("Test");
    let r = table.get(fxhash::hash32("Test"));
    assert!(r.is_none());
    assert!(table.remove("Test2").is_some());
    assert_eq!(None, table.remove("Ballbag"));
}

#[test]
fn should_handle_bulk_additions() {
    let mut table: StringTable = StringTable::new();
    let mut strs: Vec<String> = vec![];
    for i in 0..10000 {
        strs.push(format!("str-{}", i));
    }
    let start = Instant::now();
    for s in strs.into_iter() {
        let _ = table.insert(s.as_str());
    }
    println!("Inserted 10000 entries in {:?}", start.elapsed());
    assert!(table.contains(table.hash("str-89")));
    assert!(table.contains(table.hash("str-2034")));
    assert!(table.contains(table.hash("str-7777")));
    assert!(table.contains(table.hash("str-1")));
}
