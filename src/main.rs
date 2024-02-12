use std::process::Command;
use std::string::String;

use nu_plugin::{serve_plugin, LabeledError, Plugin,EvaluatedCall,MsgPackSerializer};
use nu_protocol::{Value, PluginSignature, Type,Category, Record};

struct DpkgTable;

struct Package <'a>  {
  status:  &'a str,
  name:  &'a str,
  version:  &'a  str,
  architecture:  &'a  str,
  description:  &'a str      
}

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
        
            let output = Command::new("/usr/bin/dpkg")
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
        
            // This loop parses ecach line

            for line in lines {
                //println!("line= {}", line);
                
                if just_found {
                   just_found= false;
                   
                   // In the line immediately after the one begining with ||/,
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
                    just_found = true;
                    found = true;
                }
        
                // We ignore all the lines befor that

                if !found { 
                    continue };
                if line.is_empty() {continue};
        
                let mut previous_break;

                // Eacch line corresponds to a package. 

                let mut package= Package{
                    status: "",
                    name: "",
                    version: "",
                    architecture: "",
                    description: ""
                };

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
                    let slice_trimmed = slice.trim_end();
                    if i==1 {
                        package.status=slice_trimmed;
                    }
                    else if i==2 {
                        package.name= slice_trimmed;
                    }
                    else if i==3 {
                        package.version= slice_trimmed;
                    }
                    else if i==4 {
                        package.architecture= slice_trimmed;
                    }
                    previous_break = new_break;
                              
                };
                let start= *previous_break+1;
                let slice = &line[start..];
                let slice_trimmed = slice.trim_end();
                package.description= slice_trimmed;
                
                // Finally, we convert the package to a record that nushell can read.

                let mut record = Record::new();
                let value = Value::String { val:package.status.to_string() ,internal_span: tag};
                record.push("status",value);
                let value = Value::String { val: package.name.to_string(),internal_span: tag};
                record.push("name",value);
                let value = Value::String { val: package.version.to_string(),internal_span: tag};
                record.push("version",value);
                let value = Value::String { val: package.architecture.to_string(),internal_span: tag};
                record.push("architecture",value);
                let value = Value::String { val: package.description.to_string(),internal_span: tag};
                record.push("description",value);

                // and we add it to the table

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

