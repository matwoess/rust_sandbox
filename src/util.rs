use std::io;
use std::io::Write;

pub(crate) fn prompt_for_choice() -> usize {
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