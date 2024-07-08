use std::process;


pub fn print_usage_and_exit() {
    println!("Unknown command");
    println!("Usage: \n
        - list - list all employees
        - list-by-department - list all employees by department
        - add \"Employee Name\" to \"Deparnent Name\"
    ");

    process::exit(1);
}

/// Parses commands from the given string.
/// 
/// This function takes input, splits it, cleans up, and splits on commands.
pub fn parse_commands_and_quote_values(s: &str) -> Vec<String> {
    // Collect quotted("") strings into owned vector
    // or a single string
    let mut commands: Vec<String> = s
        .split('"')
        .map(str::trim)
        .map(str::to_owned)
        .filter(|s| !s.is_empty())
        .collect();

    // If there are no quotted strings
    if commands.len() == 1 {
        commands = commands[0]
            .split_whitespace()
            .map(|s| s.to_owned().to_lowercase())
            .collect();
    }
    // In case string quotes are met - parse the commands separately
    // and then join them altogether in order:
    // - commands splitted by spaces
    // - quotted strings splitted by quotes
    else {
        let mut splitted_commands: Vec<String> = vec![];
        // By its nature that command (or group of commands)
        // will go before quotted string and they will be in pairs
        for ch in commands.chunks(2) {
            if ch.len() != 2 {
                continue
            }
                
            splitted_commands.append(&mut ch[0]
                .trim()
                .to_lowercase()
                .split_whitespace().map(str::to_owned).collect());
            splitted_commands.push(ch[1].to_owned());
        }
        commands = splitted_commands;
    }

    commands
}