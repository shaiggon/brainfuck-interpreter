use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;

fn read_program_bytes(path: &String) -> Vec<u8> {
    let program = fs::read_to_string(path)
        .expect("Should have been able to read the file").into_bytes();

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
// Current instruction pointer must be in a bracket that matches the direction
fn find_matching_bracket(program: &Vec<u8>, instruction_pointer: usize, direction: SearchDirection) -> usize {
    let direction_integer = direction as i32;
    let mut i = instruction_pointer as i32 + direction_integer;
    let mut matching_brackets = 1;
    let program_length = program.len() as i32;
    while matching_brackets > 0 && i < program_length && i >= 0 {
        i += direction_integer;
        let ins = program[i as usize] as char;
        match ins {
            '[' => matching_brackets += direction_integer,
            ']' => matching_brackets -= direction_integer,
            _ => (),
        }
    }
    return i as usize;
}

fn main() -> io::Result<()> {
    let mut data_pointer = 0;
    let mut instruction_pointer = 0;
    let mut data: Vec<u8> = vec![0; 30000];

    let program_path = std::env::args().nth(1).expect("No path given");
    let program = read_program_bytes(&program_path);

    while instruction_pointer < program.len() {
        let instruction = program[instruction_pointer] as char;

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
                    instruction_pointer = find_matching_bracket(&program, instruction_pointer, SearchDirection::Forward);
                }
            },
            ']' => {
                if data[data_pointer] != 0 {
                    instruction_pointer = find_matching_bracket(&program, instruction_pointer, SearchDirection::Backwards);
                }
            },
            _ => (),
        }

        instruction_pointer += 1;
    }

    Ok(())
}
