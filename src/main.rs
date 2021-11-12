use std::process::exit;

mod notes;
mod util;

struct Module<'a> {
    name: &'a str,
    main_fn: fn(),
}

fn main() {
    println!("Available modules:");
    let mods = [
        Module {
            name: "notes",
            main_fn: notes::main as fn(),
        },
        Module {
            name: "quit",
            main_fn: quit as fn(),
        },
    ];
    let chosen = get_module_to_run(&mods);
    let module = mods.get(chosen).expect("Invalid option!");
    (module.main_fn)();
}

fn get_module_to_run(mods: &[Module; 2]) -> usize {
    for (i, module) in mods.iter().enumerate() {
        println!("{}) {}", i, module.name);
    }
    print!("Module to run?: ");
    util::prompt_for_choice()
}

fn quit() {
    println!("> exiting the program...");
    exit(0);
}
