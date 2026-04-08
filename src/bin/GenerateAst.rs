// This file should generate AST boiler plate  
use std::env;
use std::fs;
use std::process;
use std::fs::File;
use std::io:: {BufWriter, Write};
use project_root::get_project_root;


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

    // searches for the project root and creates a file in the specified path
    let root = get_project_root().expect("Failed to find project root");
    let path = root.join("src").join("lox").join("expressions.rs");
    let contents = fs::read_to_string(init.file_description_path)?;
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let _ = writeln!(writer, "use crate::lox::Token;" );
    let _ = writeln!(writer, "use crate::lox::Object;" );
    let _ = writeln!(writer, "");
    let _ = writeln!(writer, "");
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
}

impl DefineAST {
    fn build (mut args: impl Iterator<Item = String>) -> Result<DefineAST, &'static str> {
        args.next();
        let file_description_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file description path"), 
        };

        Ok(DefineAST{
            file_description_path: file_description_path, 
        })
    }
}


