use std::process::Command;
use serde_json

fn run_recognition(img_path: &str) -> String {
    let output = Command::new("python3")
        .arg("../recognizer/recgonize.py")
        .arg(img_path)
        .output()
        .expect("Failed to run the python script");

    String::from_utf8(output.stdout).unwrap()
}

fn main() {
    let result = run_recognition("../IMG_8882.jpg");
    println!("{result}");
}
