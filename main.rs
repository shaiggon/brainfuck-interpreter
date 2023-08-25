use std::io;
use std::io::Read;
use std::io::Write;
//use std::env;
use std::fs;

fn read_program(path: &String) -> String {
    let program = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    return program;
}

fn read_byte() -> u8 {
    let input: u8 = std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8)
        .expect("Not a valid byte");

    return input;
}

fn main() -> io::Result<()> {
    let mut data_pointer = 0;
    let mut instruction_pointer = 0;
    let mut data: Vec<u8> = vec![0; 30000];

    let program_path = std::env::args().nth(1).expect("No path given");
    let program = read_program(&program_path);
    let program_bytes = program.as_bytes();

    println!("{}", program_bytes.len());

    /*let mut stdin = std::io::stdin();
    let mut byte = [0];
    stdin.read_char(&mut byte).unwrap();
    println!("The byte from stdin is: {}", byte[0]);*/

    /*let input: Option<u8> = std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8);*/

    //let input = read_byte();

    while instruction_pointer < program.len() {
        let instruction = program_bytes[instruction_pointer] as char;

        match instruction {
            '>' => {
                data_pointer += 1;
                instruction_pointer += 1;
            },
            '<' => {
                data_pointer -= 1;
                instruction_pointer += 1;
            },
            '+' => {
                data[data_pointer] += 1;
                instruction_pointer += 1;
            },
            '-' => {
                data[data_pointer] -= 1;
                instruction_pointer += 1;
            },
            '.' => {
                io::stdout().write_all(&[data[data_pointer]])?;
                instruction_pointer += 1;
            },
            ',' => {
                data[data_pointer] = read_byte();
                instruction_pointer += 1;
            }
            _ => instruction_pointer += 1,

        }

        println!("{}", instruction);
        //instruction_pointer += 1;
    }

    //println!("byte is: {:?}", input);

    println!("{}", program_path);
    //println!("{}", program);
    Ok(())
}
