use std::io;
use std::io::Read;
use std::io::Write;
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

    while instruction_pointer < program.len() {
        let instruction = program_bytes[instruction_pointer] as char;

        match instruction {
            '>' => {
                data_pointer += 1;
            },
            '<' => {
                data_pointer -= 1;
            },
            '+' => {
                data[data_pointer] += 1;
            },
            '-' => {
                data[data_pointer] -= 1;
            },
            '.' => {
                io::stdout().write_all(&[data[data_pointer]])?;
            },
            ',' => {
                data[data_pointer] = read_byte();
            },
            '[' => {
                if data[data_pointer] == 0 {
                    let mut i = instruction_pointer + 1;
                    let mut matching_brackets = 1;
                    while matching_brackets > 0 && i < program.len() {
                        let ins = program_bytes[i] as char;
                        match ins {
                            '[' => matching_brackets += 1,
                            ']' => matching_brackets -= 1,
                            _ => (),
                        }
                        i += 1;
                    }
                    instruction_pointer = i - 1;
                }
            },
            ']' => {
                if data[data_pointer] != 0 {
                    let mut i = instruction_pointer - 1;
                    let mut matching_brackets = 1;
                    while matching_brackets > 0 && i >= 0 {
                        let ins = program_bytes[i] as char;
                        match ins {
                            '[' => matching_brackets -= 1,
                            ']' => matching_brackets += 1,
                            _ => (),
                        }
                        i -= 1;
                    }
                    instruction_pointer = i + 1;
                }
            },
            _ => (),
        }

        instruction_pointer += 1;
    }

    Ok(())
}
