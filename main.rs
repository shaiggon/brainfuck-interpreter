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

enum SearchDirection {
    Forward = 1,
    Backwards = -1
}

// Return instruction pointer position
fn find_matching_bracket(program_bytes: &[u8], instruction_pointer: usize, direction: SearchDirection) -> usize {
    let direction_integer = direction as i32;
    let mut i = (instruction_pointer as i32 + direction_integer) as usize;
    let mut matching_brackets = 1;
    while matching_brackets > 0 && i < program_bytes.len() {
        let ins = program_bytes[i] as char;
        match ins {
            '[' => matching_brackets += direction_integer,
            ']' => matching_brackets -= direction_integer,
            _ => (),
        }
        i = (i as i32 + direction_integer) as usize;
    }
    return (i as i32 - direction_integer) as usize;
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
                    instruction_pointer = find_matching_bracket(program_bytes, instruction_pointer, SearchDirection::Forward);
                }
            },
            ']' => {
                if data[data_pointer] != 0 {
                    instruction_pointer = find_matching_bracket(program_bytes, instruction_pointer, SearchDirection::Backwards);
                }
            },
            _ => (),
        }

        instruction_pointer += 1;
    }

    Ok(())
}
