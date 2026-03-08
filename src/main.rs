use std::process::Command;
use std::string::String;

use nu_plugin::{serve_plugin, EngineInterface, Plugin, PluginCommand, SimplePluginCommand, EvaluatedCall, MsgPackSerializer};
use nu_protocol::{LabeledError, Value, Signature, Type, Category, Record};

struct DpkgTable;

struct Package <'a>  {
  status:  &'a str,
  name:  &'a str,
  version:  &'a  str,
  architecture:  &'a  str,
  description:  &'a str
}

impl Plugin for DpkgTable {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(DpkgTableCommand)]
    }
}

struct DpkgTableCommand;

impl SimplePluginCommand for DpkgTableCommand {
    type Plugin = DpkgTable;

    fn name(&self) -> &str {
        "dpkgtable"
    }

    fn description(&self) -> &str {
        "creates a table of all known packages in a Debian system."
    }

    fn signature(&self) -> Signature {
        Signature::build("dpkgtable")
            .input_output_types(vec![(Type::Nothing, Type::Table(vec![("name".to_string(), Type::String)].into_boxed_slice()))])
            .category(Category::System)
    }

    fn run(
        &self,
        _plugin: &DpkgTable,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {

            let tag = call.head;

            // https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output

            let output = Command::new("/usr/bin/dpkg")
                .arg("--list")
                .output()
                .expect("failed to execute dpkg");

            // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust

            let binding = String::from_utf8_lossy(&output.stdout);
            let lines = binding.split("\n");

            assert!(output.status.success());

            let mut just_found = false;
            let mut found = false;

            let mut column_breaks: Vec<usize> = Vec::new();
            column_breaks.push(0);

            // A table is a list of records
            // Internally, a vector of values

            let mut list: Vec<Value>= Vec::new();

            // This loop parses ecach line

            for line in lines {

                if just_found {
                   just_found= false;

                   // In the line immediately after the one begining with ||/,
                   // we look for the columns with - that indicate columns breaks
                   // and store the indexes

                   for (i, c) in line.chars().enumerate() {
                        if c=='-'{
                            column_breaks.push(i)
                        }
                    };
                    continue;
                }
                // We look for the line starting with ||/ that has the column names
                if line.starts_with("||/") {
                    just_found = true;
                    found = true;
                    continue;
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
                record.push("status", Value::string(package.status, tag));
                record.push("name", Value::string(package.name, tag));
                record.push("version", Value::string(package.version, tag));
                record.push("architecture", Value::string(package.architecture, tag));
                record.push("description", Value::string(package.description, tag));

                // and we add it to the table

                list.push(Value::record(record, tag));
            }
            Ok(
                Value::list(list, tag)
            )
        }
}

fn main() {
    serve_plugin(&DpkgTable, MsgPackSerializer {})
}
