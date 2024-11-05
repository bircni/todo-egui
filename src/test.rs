#![allow(clippy::unwrap_used)]
use crate::data::List;

#[test]
fn test_new_with_valid_json() {
    let json = Some(r#"{"categories": []}"#.to_string());
    let result = List::new(json);
    assert!(result.is_ok());
    let list = result.unwrap();
    assert!(list.categories.is_empty());
}

#[test]
fn test_new_with_invalid_json() {
    let json = Some(r"{}".to_string());
    let result = List::new(json);
    assert!(result.is_err());
}

#[test]
fn test_new_with_none_json() {
    let json = None;
    let result = List::new(json);
    assert!(result.is_ok());
    let list = result.unwrap();
    assert_eq!(list, List::default());
}
