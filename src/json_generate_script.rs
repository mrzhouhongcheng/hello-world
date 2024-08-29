use std::{collections::HashMap, fs, io::Write, path::Path};

use uuid::Uuid;

const OUT_PATH: &'static str = r"result.txt";

pub fn format_json() {
    let lins = fs::read_to_string(Path::new(OUT_PATH)).expect("read to string failed");

    let mut content: String = String::new();

    for line in lins.lines() {
        if line.contains("$oid") || line.contains("$date") {
            continue;
        }
        if line.contains("updateUserId") || line.contains("createUserId") {
            continue;
        }
        content.push_str(line);
        content.push('\n');
    }

    let path: &Path = Path::new(OUT_PATH);
    if path.exists() {
        fs::remove_file(path).unwrap();
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(OUT_PATH)
        .unwrap();
    file.write(content.as_bytes()).unwrap();
}

pub fn generate_file() {
    let content = fs::read_to_string(Path::new(OUT_PATH)).unwrap();
    let mut data: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&content).unwrap();

    for item in &mut data {
        let id = generate_id();
        item.insert("_id".to_string(), serde_json::to_value(&id).unwrap());
        item.insert("id".to_string(), serde_json::to_value(&id).unwrap());
    }
    println!("send content to file");
    fs::remove_file(OUT_PATH).unwrap();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(OUT_PATH)
        .unwrap();
    file.write(serde_json::to_string_pretty(&data).unwrap().as_bytes())
        .unwrap();
}

#[test]
fn test_generate_file() {
    let path = Path::new(OUT_PATH);
    if path.exists() {
        fs::remove_file(OUT_PATH).unwrap();
    }
    let content = r#"[{
    "name": "Alice",
    "age": 20,
    "email": "alice@example.com"}]
    "#;
    let mut file = fs::OpenOptions::new().create(true).write(true).open(Path::new(OUT_PATH)).unwrap();
    file.write(content.as_bytes()).unwrap();
    assert!(path.exists());

    generate_file();
    // 读取这个文件的内容
    let file_content = fs::read_to_string(OUT_PATH).unwrap();
    assert!(file_content.contains("id"));
    assert!(file_content.contains("_id"));
    fs::remove_file(OUT_PATH).unwrap();
}


pub fn generate_id() -> String {
    let id = Uuid::new_v4();

    let id = id.to_string();

    id.replace("-", "")
}

#[test]
fn generate_id_test() {
    println!("generate id test");
    let id = generate_id();
    println!("generate id : {}", id);
    assert!(id != "");
    assert!(!id.contains("-"))
}

#[test]
fn test_string_default_value() {
    let id = String::new();
    assert!(id == "");
}
