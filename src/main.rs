use std::process::Command;
use std::string::String;

fn main() {

// https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output

let output = Command::new("dpkg")
                     .arg("--list")
                     .output()
                     .expect("failed to execute process");

// println!("status: {}", output.status);
// println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
// println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

// https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust

let binding = String::from_utf8_lossy(&output.stdout);
let lines = binding.split("\n");

assert!(output.status.success());

for line in lines {
    println!("{}", line);
    if line.starts_with("||/") {
        println!("Encontrada! \n"); 
        let parts = line.split_whitespace();
        let collection: Vec<&str> = parts.collect();
        for columna in collection.iter() {
            println!("{}", columna);
        }
    } else {
        println!("NO")
    }
}

//let collection = parts.collect::<Vec<&str>>();
//dbg!(collection);

// let collection: Vec<&str> = parts.collect();
// dbg!(collection);


}
