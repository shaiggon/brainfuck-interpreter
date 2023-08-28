use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

fn read_program_bytes(path: &String) -> Vec<u8> {
    fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .into_bytes()
}

fn read_byte() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .expect("Not a valid byte")
}

enum SearchDirection {
    Forward = 1,
    Backwards = -1,
}

// Return instruction pointer position
// Current instruction pointer must be in a bracket that matches the direction
fn find_matching_bracket(
    program: &Vec<u8>,
    instruction_pointer: usize,
    direction: SearchDirection,
) -> usize {
    let direction_integer = direction as i32;
    let mut i = instruction_pointer as i32;
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
    i as usize
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
            }
            '<' => {
                data_pointer -= 1;
            }
            '+' => {
                data[data_pointer] += 1;
            }
            '-' => {
                data[data_pointer] -= 1;
            }
            '.' => {
                io::stdout().write_all(&[data[data_pointer]])?;
            }
            ',' => {
                data[data_pointer] = read_byte();
            }
            '[' => {
                if data[data_pointer] == 0 {
                    instruction_pointer = find_matching_bracket(
                        &program,
                        instruction_pointer,
                        SearchDirection::Forward,
                    );
                }
            }
            ']' => {
                if data[data_pointer] != 0 {
                    instruction_pointer = find_matching_bracket(
                        &program,
                        instruction_pointer,
                        SearchDirection::Backwards,
                    );
                }
            }
            _ => (),
        }

        instruction_pointer += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::find_matching_bracket;

    #[test]
    fn find_correct_closing_bracket() {
        let starting_index: usize = 6;
        let expected_end: usize = 15;
        let program = String::from("[[[[]][[[]][[]]]]]").into_bytes();
        let matching_bracket =
            find_matching_bracket(&program, starting_index, crate::SearchDirection::Forward);
        assert_eq!(expected_end, matching_bracket);
    }

    #[test]
    fn find_correct_opening_bracket() {
        let expeced_starting_index: usize = 6;
        let end_index: usize = 15;
        let program = String::from("[[[[]][[[]][[]]]]]").into_bytes(); // [[[[]][[[]][[]]]]]
        let matching_bracket =
            find_matching_bracket(&program, end_index, crate::SearchDirection::Backwards);
        assert_eq!(expeced_starting_index, matching_bracket);
    }
}
