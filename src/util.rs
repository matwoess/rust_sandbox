use std::io;
use std::io::Write;

pub(crate) fn prompt_for_usize() -> usize {
    let mut option = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut option)
        .expect("> failed to read stdin");
    let trimmed = option.trim();
    let chosen: usize = match trimmed.parse::<usize>() {
        Ok(i) => i,
        Err(_) => {
            println!("> could not parse: {}", option);
            usize::MAX
        }
    };
    chosen
}

pub(crate) fn prompt_for_sting() -> String {
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("> failed to read stdin");
    input.trim().to_string()
}


const MIN_X: char = '0';
const MAX_X: char = '9';
const MIN_Y: char = 'a';
const MAX_Y: char = 'z';
const MIN_X_U32: u32 = MIN_X as u32;
const MAX_X_U32: u32 = MAX_X as u32;
const MIN_Y_U32: u32 = MIN_Y as u32;
const MAX_Y_U32: u32 = MAX_Y as u32;

pub(crate) fn prompt_for_coordinates() -> Result<(usize, usize), String> {
    let mut coordinate_string = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut coordinate_string)
        .expect("> failed to read stdin");
    let xy = coordinate_string.trim();
    if xy.len() != 2 {
        return Err(format!("could not parse coordinates from: {}", xy));
    }
    let chosen: (usize, usize) = {
        let x: u32 = xy.chars().nth(0).unwrap() as u32;
        let y: u32 = xy.chars().nth(1).unwrap() as u32;
        if x < MIN_X_U32 || x > MAX_X_U32 || y < MIN_Y_U32 || y > MAX_Y_U32 {
            return Err(format!("> indices out of bounds [{}-{}][{}-{}]", MIN_X, MAX_X, MIN_Y, MAX_Y));
        }
        let x: usize = (x - MIN_X_U32) as usize;
        let y: usize = (y - MIN_Y_U32) as usize;
        (x, y)
    };
    Ok(chosen)
}