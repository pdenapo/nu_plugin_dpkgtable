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

    let mut just_found = false;
    let mut found = false;

    let mut column_breaks: Vec<usize> = Vec::new();
    column_breaks.push(0);

    let mut packages: Vec<Vec<&str>> = Vec::new();

    for line in lines {
        println!("line= {}", line);
        
        if just_found {
           just_found= false;
           
           // we look for the columns with - that indicate columns breaks
           // and store the indexes

           for (i, c) in line.chars().enumerate() {
                if c=='-'{
                    println!("Break found at column {} ",i);
                    column_breaks.push(i)
                }
            };

        }
        // We look for the line starting with ||/ that has the column names
        if line.starts_with("||/") {
            println!("¡Found! \n");
            let parts = line.split_whitespace();
            let column_names: Vec<&str> = parts.collect();
            just_found = true;
            found = true;
            dbg!(column_names);
        }

        if !found { 
            println!("ignorada");
            continue };
        if line.is_empty() {continue};

        let mut columns: Vec<&str> = Vec::new();
        let mut previous_break;
        previous_break = &column_breaks[0];
        for (i, new_break) in column_breaks.iter().enumerate() {
            if i==0 {continue} ; 
            println!("Break {}",new_break);
            let start;
            if *previous_break > 0 {
                start= *previous_break+1
            } 
            else
            { start=0}; 
            let slice = &line[start..*new_break];
            println!("slice {}",slice);
            previous_break = new_break;
            columns.push(slice.trim_end());
        };
        let start= *previous_break+1;
        let slice = &line[start..];
        println!("final slice {}",slice);
        columns.push(slice.trim_end());
        dbg!(&columns);
        packages.push(columns);
    }

    dbg!(packages);
}

// We generate the table of packages. It remains to see how to interact with nushell.