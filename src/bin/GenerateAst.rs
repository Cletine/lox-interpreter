// This file should generate AST boiler plate  
use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::fs::File;
use std::io:: {BufWriter, Write};
use scanner_rust::ScannerStr;
use lox_interpreter::error;


fn main() {
    let ast: DefineAST = DefineAST::build(env::args()).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = generate_boilerplate(ast) {
        println!("Error : {e}");
        process::exit(1)
    }
}

fn generate_boilerplate(init : DefineAST) -> std::io::Result<()> {

    let contents = fs::read_to_string(init.file_description_path)?;
    let file = File::create("expressions.rs")?;
    let mut writer = BufWriter::new(file);

    let _ = writeln!(writer, "pub enum Expr {{" );

    for line in contents.lines()  {
        println!("line: {}", line);
        let line_as_vec : Vec<&str> = line.split(':').collect();
        let class_name : &str = line_as_vec[0].trim();
        let fields : Vec<&str> = line_as_vec[1].trim().split(',').collect();
        println!("{}" , class_name);

        let _ = writeln!(writer, "\t{class_name} {{");

        for field in fields {
            let field_as_vec : Vec<&str> = field.trim().split(' ').collect(); 
            let field_name : &str = field_as_vec[1].trim();
            let field_type : &str = field_as_vec[0].trim();

            println!("field_name: {}" , field_name);
            println!("field_type: {}" , field_type);
            let _ = match field_type {
                "Expr" => writeln!(writer,  "    {field_name}: Box<Expr>,"),
                "Token" => writeln!(writer, "    {field_name}: Token,"),
                "Object" => writeln!(writer,"    {field_name}: Object,"),
                _ => panic!("Unexpected field type: {field_type}"),
            };
        }

        let _ = writeln!(writer, "    }},");
    }

    let _ = writeln!(writer, "}}");
    writer.flush()?;
    Ok(())
}

struct DefineAST {
    pub file_description_path: String,
    pub file_destination_path: String,
}

impl DefineAST {
    fn build (mut args: impl Iterator<Item = String>) -> Result<DefineAST, &'static str> {
        args.next();
        let file_description_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file description path"), 
        };

        let file_destination_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file expression path")
        };

        Ok(DefineAST{
            file_description_path: file_description_path, 
            file_destination_path: file_destination_path 
        })
    }
}


