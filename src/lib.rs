use regex::Regex;
use serde_json::{self, Value};

fn remove_newlines(str: &str) -> std::string::String {
    let re = Regex::new(r#"\\n"#).unwrap();
    let res = re.replace_all(str, "").to_string();
    res
}
fn remove_slash(str: &str) -> std::string::String {
    let re = Regex::new(r#""\\"#).unwrap();
    let res = re.replace_all(str, "").to_string();
    res
}
fn remove_doubleap(str: &str) -> std::string::String {
    let re = Regex::new(r#""""#).unwrap();
    let res = re.replace_all(str, "").to_string();
    res
}
fn remove_doubleap2(str: &str) -> std::string::String {
    let re = Regex::new(r#""\\""#).unwrap();
    let res = re.replace_all(str, "\"").to_string();
    res
}

fn remove_doubleap3(str: &str) -> std::string::String {
    let re = Regex::new(r#"\\""#).unwrap();
    let res = re.replace_all(str, "\"").to_string();
    res
}

fn remove_apstart(str: &str) -> std::string::String {
    let re = Regex::new(r#"\\""#).unwrap();
    let res = re.replace_all(str, "\"").to_string();
    res
}
fn remove_apend(str: &str) -> std::string::String {
    let re = Regex::new(r#"\\}"#).unwrap();
    let res = re.replace_all(str, "\"}").to_string();
    res
}
fn remove_brstart(str: &str) -> std::string::String {
    let re = Regex::new(r#"\]"#).unwrap();
    let res = re.replace_all(str, "]").to_string();
    res
}
fn remove_brend(str: &str) -> std::string::String {
    let re = Regex::new(r#"\}"]"#).unwrap();
    let res = re.replace_all(str, "}]").to_string();
    res
}
fn remove_brend2(str: &str) -> std::string::String {
    let re = Regex::new(r#"\\]"#).unwrap();
    let res = re.replace_all(str, "\"]").to_string();
    res
}
fn remove_apbr(str: &str) -> std::string::String {
    let re = Regex::new(r#""\{"#).unwrap();
    let res = re.replace_all(str, "{").to_string();
    res
}
fn trimstr(str: &str) -> std::string::String {
    (&str.trim_end()).to_string()
}

pub fn parse(input: &str) -> Value {
    let mut temp = input.to_string();
    temp = remove_newlines(&temp);
    temp = remove_slash(&temp);
    temp = remove_doubleap(&temp);
    temp = remove_doubleap2(&temp);
    temp = remove_doubleap3(&temp);
    temp = remove_apstart(&temp);
    temp = remove_apend(&temp);
    temp = remove_brstart(&temp);
    temp = remove_brend(&temp);
    temp = remove_apbr(&temp);
    temp = remove_brend2(&temp);
    temp = trimstr(&temp);
    //std::fs::write("log.txt", &temp).expect("Unable to write file");

    let json: serde_json::Value =
        serde_json::from_str(&temp[0..temp.len() - 1]).expect("JSON was not well-formatted");
    json
}
#[cfg(test)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::Read;

#[test]
fn unstringify() {
    let filename = "./log/block3.json";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let json = parse(&content);
            assert!(json["blockHeight"] == 115724601);
        }
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        }
    }
}
