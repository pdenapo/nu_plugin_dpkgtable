use std::process::Command;
use std::string::String;

use nu_plugin::{serve_plugin, LabeledError, Plugin, JsonSerializer, EvaluatedCall,MsgPackSerializer};
use nu_protocol::{Value, PluginSignature, Type,Category, Record};

struct DpkgTable;

impl Plugin for DpkgTable {
    fn signature(&self) -> Vec<PluginSignature> {
        
        vec![PluginSignature::build("dpkgtable")
            .usage("creates a table of all known packages in a Debian system.")
            .input_output_types(vec![(Type::Nothing,Type::Table(vec![("name".to_string(),Type::String)]))])
            .category(Category::System)]
    }

    fn run(
        &mut self,
        _name: &str,
        _config: &Option<Value>,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
            
            let tag = call.head;

            // https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output
        
            let output = Command::new("dpkg")
                .arg("--list")
                .output()
                .expect("failed to execute dpkg");
        
            // println!("status: {}", output.status);
            // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        
            // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
        
            let binding = String::from_utf8_lossy(&output.stdout);
            let lines = binding.split("\n");
            
            assert!(output.status.success());
            //dbg!("dpkg successful");

            let mut just_found = false;
            let mut found = false;
        
            let mut column_breaks: Vec<usize> = Vec::new();
            column_breaks.push(0);
        
            // A table is a list of records
            // Internally, a vector of values
        
            let mut list: Vec<Value>= Vec::new();
            let mut column_names: Vec<&str>= vec!["status","name","version","architecture","description"];
        
            for line in lines {
                //println!("line= {}", line);
                
                if just_found {
                   just_found= false;
                   
                   // we look for the columns with - that indicate columns breaks
                   // and store the indexes
        
                   for (i, c) in line.chars().enumerate() {
                        if c=='-'{
                            //println!("Break found at column {} ",i);
                            column_breaks.push(i)
                        }
                    };
                    continue; 
                }
                // We look for the line starting with ||/ that has the column names
                if line.starts_with("||/") {
                    //println!("¡Found! \n");
                    //let parts = line.split_whitespace();
                    //column_names = parts.collect();
                    just_found = true;
                    found = true;
                    //dbg!(column_names.clone());
                }
        
                if !found { 
                    //println!("ignorada");
                    continue };
                if line.is_empty() {continue};
        
                let mut previous_break;
                let mut record = Record::new();
                previous_break = &column_breaks[0];
                for (i, new_break) in column_breaks.iter().enumerate() {
                    if i==0 {continue} ; 
                    //println!("Break {}",new_break);
                    let start;
                    if *previous_break > 0 {
                        start= *previous_break+1
                    } 
                    else
                    { start=0}; 
                    let slice = &line[start..*new_break];
                    //dbg!("slice {}",slice);
                    previous_break = new_break;
                    let value = Value::String { val: slice.trim_end().to_string(),internal_span: tag};
                    record.push(column_names[i],value);           
                };
                let start= *previous_break+1;
                let slice = &line[start..];
                //println!("final slice {}",slice);
                let value = Value::String { val: slice.trim_end().to_string(),internal_span: tag};
                record.push("final",value);
                list.push(Value::Record{val:record,internal_span:tag});
            }
            Ok(
                Value::list(list, tag)           
            )          
        }
}

fn main() {
    serve_plugin(&mut DpkgTable {}, MsgPackSerializer {})
}

// We generate the table of packages. It remains to see how to interact with nushell.