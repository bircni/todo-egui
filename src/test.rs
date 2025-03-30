use std::{fs, path::Path};

use crate::data::{Category, Item, List};

#[test]
fn test_list() {
    fs::create_dir_all("test-dir").unwrap_or_else(|e| {
        panic!("Failed to create test directory: {e:?}");
    });
    let mut list = List::default();
    assert_eq!(list.categories.len(), 0);
    list.categories.push(Category::default());
    assert_eq!(list.categories.len(), 1);
    assert_eq!(list.categories[0].name, "");
    assert_eq!(list.categories[0].items.len(), 0);

    let item = Item::default();
    assert_eq!(item.name, "");
    assert!(!item.todo);
    assert_eq!(item.notes, "");
    list.categories[0].items.push(item);
    assert_eq!(list.categories[0].items.len(), 1);

    let write_result = list.write(Path::new("test-dir/test.json"), false);
    assert!(
        write_result.is_ok(),
        "Failed to write list to file: {:?}",
        write_result.err()
    );
    let load_result = list.load(Path::new("test-dir/test.json"));
    fs::remove_file("test-dir/test.json").unwrap_or_else(|e| {
        panic!("Failed to remove test file: {e:?}");
    });
    assert!(
        load_result.is_ok(),
        "Failed to load list from file: {:?}",
        load_result.err()
    );

    assert_eq!(list.categories.len(), 1);
    assert_eq!(list.categories[0].name, "");
    assert_eq!(list.categories[0].items.len(), 1);
    assert_eq!(list.categories[0].items[0].name, "");
    assert!(!list.categories[0].items[0].todo);
    assert_eq!(list.categories[0].items[0].notes, "");

    // create new file
    let write_result = list.write(Path::new("test-dir/test.json"), true);
    assert!(
        write_result.is_ok(),
        "Failed to write list to file: {:?}",
        write_result.err()
    );

    let load_result = list.load(Path::new("test-dir/test.json"));
    fs::remove_file("test-dir/test.json").unwrap_or_else(|e| {
        panic!("Failed to remove test file: {e:?}");
    });
    assert!(
        load_result.is_ok(),
        "Failed to load list from file: {:?}",
        load_result.err()
    );

    assert!(
        list.categories.is_empty(),
        "List should be empty after new file"
    );

    fs::remove_dir_all("test-dir").unwrap_or_else(|e| {
        panic!("Failed to remove test directory: {e:?}");
    });
}
