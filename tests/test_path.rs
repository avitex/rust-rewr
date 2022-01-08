use std::collections::HashMap;

#[test]
fn test_path() {
    let path = rewr::path!("hello", "hello", [1], "bob");
    let mut map = HashMap::new();
    map.insert("hello".to_owned(), rewr::path::Value::Integer(1));
    let value = rewr::path::Value::Map(map);

    rewr::path::get(path, &value);
}
