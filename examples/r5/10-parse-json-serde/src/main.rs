use serde_json::Value;

fn main() {
    // ✨ How to preserve newline JSON str.
    let foo_str = r#"[
        {"id": "foo"},
        {"id": "bar"}
    ]"#;

    // ✨ How to parse json from str.
    let foo_json: Vec<Value> = serde_json::from_str::<Vec<Value>>(foo_str).unwrap();
    println!("1️⃣ foo_json = {:#?}", foo_json);

    // ✨ How to filter json elements by matched value.
    let filtered_foo_json = foo_json
        .iter()
        .filter(|v| v["id"] == "foo")
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();

    println!("2️⃣ filter_and_map_foo_json = {:#?}", filtered_foo_json);

    // ✨ How to filter json elements with filter_map by matched value.
    let filtered_foo_json = foo_json
        .iter()
        .filter_map(|v| {
            if v["id"] == "foo" {
                Some(v.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("3️⃣ filter_map_foo_json = {:#?}", filtered_foo_json);

    // ✨ How to filter json element's value with filter_map by matched value.
    let filtered_foo_value_json = foo_json
        .iter()
        .filter(|v| v["id"] == "foo")
        .filter_map(|v| v["id"].as_str())
        .collect::<Vec<_>>();

    println!("4️⃣ filtered_foo_value_json = {:#?}", filtered_foo_value_json);
}
