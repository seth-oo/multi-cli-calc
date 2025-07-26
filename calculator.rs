use std::io::{self, Write};
use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::env;

fn main() {
    // ALL VARIABLES DECLARED AT START - PROCEDURAL STYLE
    let mut program_running = true;
    let mut input = String::new();
    let mut expression = String::new();
    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut aliases: HashMap<String, String> = HashMap::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut result: f64 = 0.0;
    let mut has_error = false;
    let mut temp_string = String::new();
    let mut i: usize;
    let mut j: usize;
    let mut processed_expr = String::new();
    let mut var_name = String::new();
    let mut var_value = String::new();
    let mut parts: Vec<String> = Vec::new();
    let mut chars_vec: Vec<char> = Vec::new();
    let mut history: Vec<String> = Vec::new();
    let mut _authenticated = false;
    let mut pin = String::new();
    let correct_pin = "SETH".to_string();
    let mut audit_log = String::new();
    let mut timestamp: u64;
    let mut settings: HashMap<String, String> = HashMap::new();
    let mut precision: usize = 6;
    let mut use_colors = true;
    let mut timer_enabled = false;
    let mut start_time = Instant::now();
    let mut elapsed_time: std::time::Duration;
    let mut multi_line_mode = false;
    let mut multi_line_buffer = String::new();
    let mut script_mode = false;
    let script_lines: Vec<String> = Vec::new();
    let mut script_index: usize = 0;
    let mut constants: HashMap<String, f64> = HashMap::new();
    let mut stats_values: Vec<f64> = Vec::new();
    let mut factorial_n: u64;
    let mut factorial_result: u64;
    let mut prime_check: u64;
    let mut is_prime: bool;
    let mut result_string = String::new();

    // ADVANCED FEATURES VARIABLES
    let mut complex_real: f64 = 0.0;
    let mut complex_imag: f64 = 0.0;
    let mut bitwise_a: i64;
    let mut bitwise_b: i64;
    let mut bitwise_result: i64;
    let mut ncr_n: u64;
    let mut ncr_r: u64;
    let mut ncr_result: u64;
    let mut npr_result: u64;
    let mut fraction_num: i64;
    let mut fraction_den: i64;
    let mut angle_value: f64;
    let mut quad_a: f64;
    let mut quad_b: f64;
    let mut quad_c: f64;
    let mut quad_discriminant: f64;
    let mut quad_x1: f64;
    let mut quad_x2: f64;
    let mut linear_a: f64;
    let mut linear_b: f64;
    let mut linear_c: f64;
    let mut linear_x: f64;
    let mut memory_slots: HashMap<String, f64> = HashMap::new();
    let mut temp_vars: Vec<f64> = Vec::new();
    let mut command_chain: Vec<String> = Vec::new();
    let mut chain_index: usize = 0;
    let mut gcd_a: u64;
    let mut gcd_b: u64;
    let mut gcd_temp: u64;
    let mut current_token = String::new();
    let mut mean_sum: f64;
    let mut random_seed: u64;
    let mut random_result: f64;

    // Initialize constants
    constants.insert("pi".to_string(), std::f64::consts::PI);
    constants.insert("e".to_string(), std::f64::consts::E);
    constants.insert("tau".to_string(), 2.0 * std::f64::consts::PI);
    constants.insert("phi".to_string(), (1.0 + 5.0_f64.sqrt()) / 2.0);

    // Initialize default settings
    settings.insert("precision".to_string(), "6".to_string());
    settings.insert("colors".to_string(), "true".to_string());
    settings.insert("timer".to_string(), "false".to_string());
    settings.insert("auto_backup".to_string(), "true".to_string());
    settings.insert("menu_mode".to_string(), "false".to_string());

    // Load settings from file
    match fs::read_to_string("calculator_settings.txt") {
        Ok(content) => {
            chars_vec = content.chars().collect();
            temp_string.clear();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == '\n' {
                    if !temp_string.is_empty() {
                        if temp_string.contains('=') {
                            parts.clear();
                            let mut current_part = String::new();
                            j = 0;
                            while j < temp_string.len() {
                                let ch = temp_string.chars().nth(j).unwrap();
                                if ch == '=' {
                                    parts.push(current_part.trim().to_string());
                                    current_part.clear();
                                } else {
                                    current_part.push(ch);
                                }
                                j += 1;
                            }
                            parts.push(current_part.trim().to_string());

                            if parts.len() == 2 {
                                settings.insert(parts[0].clone(), parts[1].clone());
                            }
                        }
                        temp_string.clear();
                    }
                } else {
                    temp_string.push(chars_vec[i]);
                }
                i += 1;
            }
        }
        Err(_) => {}
    }

    // Apply loaded settings
    if let Some(prec_str) = settings.get("precision") {
        if let Ok(prec) = prec_str.parse::<usize>() {
            precision = prec;
        }
    }
    if let Some(color_str) = settings.get("colors") {
        use_colors = color_str == "true";
    }
    if let Some(timer_str) = settings.get("timer") {
        timer_enabled = timer_str == "true";
    }

    // Load variables and memory slots
    match fs::read_to_string("calculator_variables.txt") {
        Ok(content) => {
            chars_vec = content.chars().collect();
            temp_string.clear();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == '\n' {
                    if !temp_string.is_empty() {
                        if temp_string.contains('=') {
                            parts.clear();
                            let mut current_part = String::new();
                            j = 0;
                            while j < temp_string.len() {
                                let ch = temp_string.chars().nth(j).unwrap();
                                if ch == '=' {
                                    parts.push(current_part.trim().to_string());
                                    current_part.clear();
                                } else {
                                    current_part.push(ch);
                                }
                                j += 1;
                            }
                            parts.push(current_part.trim().to_string());

                            if parts.len() == 2 {
                                if let Ok(val) = parts[1].parse::<f64>() {
                                    if parts[0].starts_with("mem_") {
                                        memory_slots.insert(parts[0][4..].to_string(), val);
                                    } else {
                                        variables.insert(parts[0].clone(), val);
                                    }
                                }
                            }
                        }
                        temp_string.clear();
                    }
                } else {
                    temp_string.push(chars_vec[i]);
                }
                i += 1;
            }
        }
        Err(_) => {}
    }

    // Load aliases
    match fs::read_to_string("calculator_aliases.txt") {
        Ok(content) => {
            chars_vec = content.chars().collect();
            temp_string.clear();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == '\n' {
                    if !temp_string.is_empty() {
                        if temp_string.contains('=') {
                            parts.clear();
                            let mut current_part = String::new();
                            j = 0;
                            while j < temp_string.len() {
                                let ch = temp_string.chars().nth(j).unwrap();
                                if ch == '=' {
                                    parts.push(current_part.trim().to_string());
                                    current_part.clear();
                                } else {
                                    current_part.push(ch);
                                }
                                j += 1;
                            }
                            parts.push(current_part.trim().to_string());

                            if parts.len() == 2 {
                                aliases.insert(parts[0].clone(), parts[1].clone());
                            }
                        }
                        temp_string.clear();
                    }
                } else {
                    temp_string.push(chars_vec[i]);
                }
                i += 1;
            }
        }
        Err(_) => {}
    }

    // Check command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        expression = args[1..].join(" ");
        processed_expr = expression.clone();

        // Replace constants
        for (const_name, const_value) in &constants {
            processed_expr = processed_expr.replace(const_name, &const_value.to_string());
        }

        // Replace variables
        for (var_name_ref, var_value_ref) in &variables {
            processed_expr = processed_expr.replace(var_name_ref, &var_value_ref.to_string());
        }

        // PROCEDURAL EXPRESSION EVALUATION
        has_error = false;
        result = 0.0;

        // Simple arithmetic evaluation
        if processed_expr.contains('+') || processed_expr.contains('-') || processed_expr.contains('*') || processed_expr.contains('/') {
            // Tokenize expression
            tokens.clear();
            current_token.clear();
            chars_vec = processed_expr.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == '+' || chars_vec[i] == '-' || chars_vec[i] == '*' || chars_vec[i] == '/' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(chars_vec[i].to_string());
                } else if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.trim().to_string());
            }

            // Simple left-to-right evaluation with operator precedence
            if tokens.len() >= 3 {
                match tokens[0].parse::<f64>() {
                    Ok(first_val) => {
                        result = first_val;
                        i = 1;
                        while i < tokens.len() - 1 {
                            let operator = &tokens[i];
                            match tokens[i + 1].parse::<f64>() {
                                Ok(next_val) => {
                                    if operator == "+" {
                                        result += next_val;
                                    } else if operator == "-" {
                                        result -= next_val;
                                    } else if operator == "*" {
                                        result *= next_val;
                                    } else if operator == "/" {
                                        if next_val != 0.0 {
                                            result /= next_val;
                                        } else {
                                            has_error = true;
                                            break;
                                        }
                                    }
                                }
                                Err(_) => {
                                    has_error = true;
                                    break;
                                }
                            }
                            i += 2;
                        }
                    }
                    Err(_) => {
                        has_error = true;
                    }
                }
            } else {
                has_error = true;
            }
        } else {
            match processed_expr.parse::<f64>() {
                Ok(val) => result = val,
                Err(_) => has_error = true,
            }
        }

        if !has_error {
            if use_colors {
                println!("\x1b[32mResult: {:.precision$}\x1b[0m", result, precision = precision);
            } else {
                println!("Result: {:.precision$}", result, precision = precision);
            }
        } else {
            if use_colors {
                println!("\x1b[31mError: Invalid expression\x1b[0m");
            } else {
                println!("Error: Invalid expression");
            }
        }
        return;
    }

    // Authentication
    if use_colors {
        println!("\x1b[36m=== Advanced Procedural CLI Calculator ===\x1b[0m");
        println!("\x1b[33mEnter PIN to access calculator:\x1b[0m");
    } else {
        println!("=== Advanced Procedural CLI Calculator ===");
        println!("Enter PIN to access calculator:");
    }

    'auth_loop: loop {
        print!("PIN : ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        pin = input.trim().to_string();

        if pin == correct_pin {
            _authenticated = true;
            if use_colors {
                println!("\x1b[32mAuthentication successful!\x1b[0m");
            } else {
                println!("Authentication successful!");
            }
            break 'auth_loop;
        } else {
            if use_colors {
                println!("\x1b[31mIncorrect PIN. Try again.\x1b[0m");
            } else {
                println!("Incorrect PIN. Try again.");
            }
        }
    }

    if use_colors {
        println!("\x1b[36mWelcome! Type 'help' or 'h' for available commands.\x1b[0m");
    } else {
        println!("Welcome! Type 'help' or 'h' for available commands.");
    }
    println!();

    // MAIN PROGRAM LOOP - COMPLETELY PROCEDURAL
    'main_loop: loop {
        if !program_running {
            break 'main_loop;
        }

        if script_mode && script_index < script_lines.len() {
            expression = script_lines[script_index].clone();
            script_index += 1;

            if use_colors {
                println!("\x1b[34mScript> {}\x1b[0m", expression);
            } else {
                println!("Script> {}", expression);
            }
        } else if multi_line_mode {
            print!("... ");
            io::stdout().flush().unwrap();

            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            temp_string = input.trim().to_string();

            if temp_string == "end" {
                multi_line_mode = false;
                expression = multi_line_buffer.clone();
                multi_line_buffer.clear();
            } else {
                multi_line_buffer.push(' ');
                multi_line_buffer.push_str(&temp_string);
                continue 'main_loop;
            }
        } else {
            print!("> ");
            io::stdout().flush().unwrap();

            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            expression = input.trim().to_string();
        }

        if expression.is_empty() {
            continue 'main_loop;
        }

        // COMMAND ABBREVIATIONS - PROCEDURAL EXPANSION
        if expression == "h" { expression = "help".to_string(); }
        else if expression == "c" { expression = "clear".to_string(); }
        else if expression == "q" { expression = "quit".to_string(); }
        else if expression == "v" { expression = "vars".to_string(); }
        else if expression == "s" { expression = "save".to_string(); }
        else if expression == "l" { expression = "load".to_string(); }
        else if expression == "d" { expression = "dump".to_string(); }

        // MULTIPLE COMMANDS PER LINE - PROCEDURAL PARSING
        if expression.contains(';') {
            command_chain.clear();
            temp_string.clear();
            chars_vec = expression.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ';' {
                    if !temp_string.trim().is_empty() {
                        command_chain.push(temp_string.trim().to_string());
                    }
                    temp_string.clear();
                } else {
                    temp_string.push(chars_vec[i]);
                }
                i += 1;
            }
            if !temp_string.trim().is_empty() {
                command_chain.push(temp_string.trim().to_string());
            }

            // If we have a command chain, process the first command now
            if !command_chain.is_empty() {
                expression = command_chain[0].clone();
                if use_colors {
                    println!("\x1b[35m[Chain 1] {}\x1b[0m", expression);
                } else {
                    println!("[Chain 1] {}", expression);
                }
                // Set up to process remaining commands in subsequent iterations
                chain_index = 1;
            }
        }

        // Add to history
        if history.len() >= 100 {
            history.remove(0);
        }
        history.push(expression.clone());

        // Get timestamp
        timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Start timer if enabled
        if timer_enabled {
            start_time = Instant::now();
        }

        // COMMAND PROCESSING - PROCEDURAL BRANCHING

        // Exit commands
        if expression == "exit" || expression == "quit" {
            program_running = false;
            continue 'main_loop;
        }

        // Help command
        if expression == "help" {
            if use_colors {
                println!("\x1b[36m=== Advanced Calculator Help ===\x1b[0m");
                println!("\x1b[33mBasic Operations:\x1b[0m");
                println!("  2 + 3, 5 - 2, 4 * 6, 8 / 2, 2 ^ 3, 17 % 5");
                println!();
                println!("\x1b[33mScientific Functions:\x1b[0m");
                println!("  sqrt(16), sin(30), cos(60), tan(45), log(100), ln(2.718)");
                println!("  abs(-5), fact(5)");
                println!();
                println!("\x1b[33mComplex Numbers:\x1b[0m");
                println!("  complex 3 4");
                println!();
                println!("\x1b[33mBitwise Operations:\x1b[0m");
                println!("  AND 12 10, OR 12 10, XOR 12 10, NOT 12");
                println!("  SHL 5 2, SHR 20 2 (shift left/right)");
                println!();
                println!("\x1b[33mCombinatorics:\x1b[0m");
                println!("  nCr 5 2, nPr 5 2");
                println!();
                println!("\x1b[33mAngle Conversion:\x1b[0m");
                println!("  deg2rad 180, rad2deg 3.14159");
                println!();
                println!("\x1b[33mEquation Solving:\x1b[0m");
                println!("  quad 1 -3 2 (ax²+bx+c=0), solve 2x+3=7");
                println!();
                println!("\x1b[33mStatistics:\x1b[0m");
                println!("  mean 1 2 3 4 5, sum 1 2 3 4 5");
                println!();
                println!("\x1b[33mMemory & Storage:\x1b[0m");
                println!("  store tax = 0.075, recall tax, mem (show all)");
                println!("  $1, $2, $3 (last 3 results)");
                println!();
                println!("\x1b[33mFractions:\x1b[0m");
                println!("  frac 3 4, gcd 12 8");
                println!();
                println!("\x1b[33mRandom & Primes:\x1b[0m");
                println!("  rand, isprime 23");
                println!();
                println!("\x1b[33mCommand Shortcuts:\x1b[0m");
                println!("  h=help, c=clear, q=quit, v=vars, s=save, l=load");
                println!("  Multiple commands: x=5; y=x+3; print y");
                println!();
                println!("\x1b[33mOther Commands:\x1b[0m");
                println!("  settings, history, dump, vars, constants");
            } else {
                println!("=== Advanced Calculator Help ===");
                println!("Basic Operations:");
                println!("  2 + 3, 5 - 2, 4 * 6, 8 / 2, 2 ^ 3, 17 % 5");
                println!();
                println!("Scientific Functions:");
                println!("  sqrt(16), sin(30), cos(60), tan(45), log(100), ln(2.718)");
                println!("  abs(-5), fact(5)");
                println!();
                println!("Complex Numbers:");
                println!("  complex 3 4");
                println!();
                println!("Bitwise Operations:");
                println!("  AND 12 10, OR 12 10, XOR 12 10, NOT 12");
                println!("  SHL 5 2, SHR 20 2 (shift left/right)");
                println!();
                println!("Combinatorics:");
                println!("  nCr 5 2, nPr 5 2");
                println!();
                println!("Angle Conversion:");
                println!("  deg2rad 180, rad2deg 3.14159");
                println!();
                println!("Equation Solving:");
                println!("  quad 1 -3 2 (ax²+bx+c=0), solve 2x+3=7");
                println!();
                println!("Statistics:");
                println!("  mean 1 2 3 4 5, sum 1 2 3 4 5");
                println!();
                println!("Memory & Storage:");
                println!("  store tax = 0.075, recall tax, mem (show all)");
                println!("  $1, $2, $3 (last 3 results)");
                println!();
                println!("Fractions:");
                println!("  frac 3 4, gcd 12 8");
                println!();
                println!("Random & Primes:");
                println!("  rand, isprime 23");
                println!();
                println!("Command Shortcuts:");
                println!("  h=help, c=clear, q=quit, v=vars, s=save, l=load");
                println!("  Multiple commands: x=5; y=x+3; print y");
                println!();
                println!("Other Commands:");
                println!("  settings, history, dump, vars, constants");
            }
            println!();
            continue 'main_loop;
        }

        // Clear command
        if expression == "clear" {
            variables.clear();
            aliases.clear();
            history.clear();
            memory_slots.clear();
            temp_vars.clear();
            if use_colors {
                println!("\x1b[32mCleared all data.\x1b[0m");
            } else {
                println!("Cleared all data.");
            }
            println!();
            continue 'main_loop;
        }

        // Variables command
        if expression == "vars" {
            if variables.is_empty() {
                if use_colors {
                    println!("\x1b[33mNo variables defined.\x1b[0m");
                } else {
                    println!("No variables defined.");
                }
            } else {
                if use_colors {
                    println!("\x1b[36mVariables:\x1b[0m");
                } else {
                    println!("Variables:");
                }
                for (name, value) in &variables {
                    if use_colors {
                        println!("  \x1b[32m{}\x1b[0m = \x1b[33m{:.precision$}\x1b[0m", name, value, precision = precision);
                    } else {
                        println!("  {} = {:.precision$}", name, value, precision = precision);
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        // Constants command
        if expression == "constants" {
            if use_colors {
                println!("\x1b[36mBuilt-in Constants:\x1b[0m");
            } else {
                println!("Built-in Constants:");
            }
            for (name, value) in &constants {
                if use_colors {
                    println!("  \x1b[32m{}\x1b[0m = \x1b[33m{:.precision$}\x1b[0m", name, value, precision = precision);
                } else {
                    println!("  {} = {:.precision$}", name, value, precision = precision);
                }
            }
            println!();
            continue 'main_loop;
        }

        // Memory slots command
        if expression == "mem" {
            if memory_slots.is_empty() {
                if use_colors {
                    println!("\x1b[33mNo memory slots used.\x1b[0m");
                } else {
                    println!("No memory slots used.");
                }
            } else {
                if use_colors {
                    println!("\x1b[36mMemory Slots:\x1b[0m");
                } else {
                    println!("Memory Slots:");
                }
                for (name, value) in &memory_slots {
                    if use_colors {
                        println!("  \x1b[32m{}\x1b[0m = \x1b[33m{:.precision$}\x1b[0m", name, value, precision = precision);
                    } else {
                        println!("  {} = {:.precision$}", name, value, precision = precision);
                    }
                }
            }

            if !temp_vars.is_empty() {
                if use_colors {
                    println!("\x1b[36mTemporary Variables:\x1b[0m");
                } else {
                    println!("Temporary Variables:");
                }
                i = 0;
                while i < temp_vars.len() && i < 10 {
                    if use_colors {
                        println!("  \x1b[32m${}\x1b[0m = \x1b[33m{:.precision$}\x1b[0m", i + 1, temp_vars[temp_vars.len() - 1 - i], precision = precision);
                    } else {
                        println!("  ${} = {:.precision$}", i + 1, temp_vars[temp_vars.len() - 1 - i], precision = precision);
                    }
                    i += 1;
                }
            }
            println!();
            continue 'main_loop;
        }

        // Store command
        if expression.starts_with("store ") {
            temp_string = expression[6..].trim().to_string();
            if temp_string.contains('=') {
                parts.clear();
                let mut current_part = String::new();
                j = 0;
                while j < temp_string.len() {
                    let ch = temp_string.chars().nth(j).unwrap();
                    if ch == '=' {
                        parts.push(current_part.trim().to_string());
                        current_part.clear();
                    } else {
                        current_part.push(ch);
                    }
                    j += 1;
                }
                parts.push(current_part.trim().to_string());

                if parts.len() == 2 {
                    // Evaluate the right side
                    processed_expr = parts[1].clone();

                    // Replace constants
                    for (const_name, const_value) in &constants {
                        processed_expr = processed_expr.replace(const_name, &const_value.to_string());
                    }

                    // Replace variables
                    for (var_name_ref, var_value_ref) in &variables {
                        processed_expr = processed_expr.replace(var_name_ref, &var_value_ref.to_string());
                    }

                    match processed_expr.parse::<f64>() {
                        Ok(val) => {
                            memory_slots.insert(parts[0].clone(), val);
                            if use_colors {
                                println!("\x1b[32mStored {} = {:.precision$}\x1b[0m", parts[0], val, precision = precision);
                            } else {
                                println!("Stored {} = {:.precision$}", parts[0], val, precision = precision);
                            }
                        }
                        Err(_) => {
                            if use_colors {
                                println!("\x1b[31mError: Invalid value\x1b[0m");
                            } else {
                                println!("Error: Invalid value");
                            }
                        }
                    }
                } else {
                    if use_colors {
                        println!("\x1b[31mError: Use format 'store name = value'\x1b[0m");
                    } else {
                        println!("Error: Use format 'store name = value'");
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Use format 'store name = value'\x1b[0m");
                } else {
                    println!("Error: Use format 'store name = value'");
                }
            }
            println!();
            continue 'main_loop;
        }

        // Recall command
        if expression.starts_with("recall ") {
            temp_string = expression[7..].trim().to_string();
            if let Some(value) = memory_slots.get(&temp_string) {
                if use_colors {
                    println!("\x1b[32m{} = {:.precision$}\x1b[0m", temp_string, value, precision = precision);
                } else {
                    println!("{} = {:.precision$}", temp_string, value, precision = precision);
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Memory slot '{}' not found\x1b[0m", temp_string);
                } else {
                    println!("Error: Memory slot '{}' not found", temp_string);
                }
            }
            println!();
            continue 'main_loop;
        }

        // Print command
        if expression.starts_with("print ") {
            temp_string = expression[6..].trim().to_string();

            // Replace variables and memory slots
            for (var_name_ref, var_value_ref) in &variables {
                temp_string = temp_string.replace(var_name_ref, &var_value_ref.to_string());
            }
            for (mem_name, mem_value) in &memory_slots {
                temp_string = temp_string.replace(mem_name, &mem_value.to_string());
            }

            match temp_string.parse::<f64>() {
                Ok(val) => {
                    if use_colors {
                        println!("\x1b[32m{:.precision$}\x1b[0m", val, precision = precision);
                    } else {
                        println!("{:.precision$}", val, precision = precision);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[33m{}\x1b[0m", temp_string);
                    } else {
                        println!("{}", temp_string);
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        // SCIENTIFIC FUNCTIONS - PROCEDURAL IMPLEMENTATION
        if expression.starts_with("sqrt(") && expression.ends_with(")") {
            temp_string = expression[5..expression.len()-1].to_string();
            match temp_string.parse::<f64>() {
                Ok(val) => {
                    if val >= 0.0 {
                        result = val.sqrt();
                        temp_vars.push(result);
                        if temp_vars.len() > 10 { temp_vars.remove(0); }
                        if use_colors {
                            println!("\x1b[32msqrt({}) = {:.precision$}\x1b[0m", val, result, precision = precision);
                        } else {
                            println!("sqrt({}) = {:.precision$}", val, result, precision = precision);
                        }
                    } else {
                        if use_colors {
                            println!("\x1b[31mError: Cannot take square root of negative number\x1b[0m");
                        } else {
                            println!("Error: Cannot take square root of negative number");
                        }
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("sin(") && expression.ends_with(")") {
            temp_string = expression[4..expression.len()-1].to_string();
            match temp_string.parse::<f64>() {
                Ok(val) => {
                    result = (val * std::f64::consts::PI / 180.0).sin();
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32msin({}°) = {:.precision$}\x1b[0m", val, result, precision = precision);
                    } else {
                        println!("sin({}°) = {:.precision$}", val, result, precision = precision);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("cos(") && expression.ends_with(")") {
            temp_string = expression[4..expression.len()-1].to_string();
            match temp_string.parse::<f64>() {
                Ok(val) => {
                    result = (val * std::f64::consts::PI / 180.0).cos();
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32mcos({}°) = {:.precision$}\x1b[0m", val, result, precision = precision);
                    } else {
                        println!("cos({}°) = {:.precision$}", val, result, precision = precision);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("tan(") && expression.ends_with(")") {
            temp_string = expression[4..expression.len()-1].to_string();
            match temp_string.parse::<f64>() {
                Ok(val) => {
                    result = (val * std::f64::consts::PI / 180.0).tan();
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32mtan({}°) = {:.precision$}\x1b[0m", val, result, precision = precision);
                    } else {
                        println!("tan({}°) = {:.precision$}", val, result, precision = precision);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("log(") && expression.ends_with(")") {
            temp_string = expression[4..expression.len()-1].to_string();
            match temp_string.parse::<f64>() {
                Ok(val) => {
                    if val > 0.0 {
                        result = val.log10();
                        temp_vars.push(result);
                        if temp_vars.len() > 10 { temp_vars.remove(0); }
                        if use_colors {
                            println!("\x1b[32mlog({}) = {:.precision$}\x1b[0m", val, result, precision = precision);
                        } else {
                            println!("log({}) = {:.precision$}", val, result, precision = precision);
                        }
                    } else {
                        if use_colors {
                            println!("\x1b[31mError: Logarithm of non-positive number\x1b[0m");
                        } else {
                            println!("Error: Logarithm of non-positive number");
                        }
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("ln(") && expression.ends_with(")") {
            temp_string = expression[3..expression.len()-1].to_string();
            match temp_string.parse::<f64>() {
                Ok(val) => {
                    if val > 0.0 {
                        result = val.ln();
                        temp_vars.push(result);
                        if temp_vars.len() > 10 { temp_vars.remove(0); }
                        if use_colors {
                            println!("\x1b[32mln({}) = {:.precision$}\x1b[0m", val, result, precision = precision);
                        } else {
                            println!("ln({}) = {:.precision$}", val, result, precision = precision);
                        }
                    } else {
                        if use_colors {
                            println!("\x1b[31mError: Natural logarithm of non-positive number\x1b[0m");
                        } else {
                            println!("Error: Natural logarithm of non-positive number");
                        }
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("abs(") && expression.ends_with(")") {
            temp_string = expression[4..expression.len()-1].to_string();
            match temp_string.parse::<f64>() {
                Ok(val) => {
                    result = val.abs();
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32mabs({}) = {:.precision$}\x1b[0m", val, result, precision = precision);
                    } else {
                        println!("abs({}) = {:.precision$}", val, result, precision = precision);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("fact(") && expression.ends_with(")") {
            temp_string = expression[5..expression.len()-1].to_string();
            match temp_string.parse::<u64>() {
                Ok(val) => {
                    if val <= 20 {
                        factorial_n = val;
                        factorial_result = 1;
                        i = 1;
                        while i <= factorial_n as usize {
                            factorial_result *= i as u64;
                            i += 1;
                        }
                        result = factorial_result as f64;
                        temp_vars.push(result);
                        if temp_vars.len() > 10 { temp_vars.remove(0); }
                        if use_colors {
                            println!("\x1b[32m{}! = {}\x1b[0m", factorial_n, factorial_result);
                        } else {
                            println!("{}! = {}", factorial_n, factorial_result);
                        }
                    } else {
                        if use_colors {
                            println!("\x1b[31mError: Factorial too large (max 20)\x1b[0m");
                        } else {
                            println!("Error: Factorial too large (max 20)");
                        }
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        // COMPLEX NUMBER OPERATIONS
        if expression.starts_with("complex ") {
            temp_string = expression[8..].trim().to_string();
            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if tokens.len() == 2 {
                match (tokens[0].parse::<f64>(), tokens[1].parse::<f64>()) {
                    (Ok(real), Ok(imag)) => {
                        complex_real = real;
                        complex_imag = imag;
                        if use_colors {
                            if complex_imag >= 0.0 {
                                println!("\x1b[32mComplex: {:.precision$} + {:.precision$}i\x1b[0m", complex_real, complex_imag, precision = precision);
                            } else {
                                println!("\x1b[32mComplex: {:.precision$} - {:.precision$}i\x1b[0m", complex_real, -complex_imag, precision = precision);
                            }
                        } else {
                            if complex_imag >= 0.0 {
                                println!("Complex: {:.precision$} + {:.precision$}i", complex_real, complex_imag, precision = precision);
                            } else {
                                println!("Complex: {:.precision$} - {:.precision$}i", complex_real, -complex_imag, precision = precision);
                            }
                        }
                    }
                    _ => {
                        if use_colors {
                            println!("\x1b[31mError: Invalid complex number format\x1b[0m");
                        } else {
                            println!("Error: Invalid complex number format");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Use format 'complex real imag'\x1b[0m");
                } else {
                    println!("Error: Use format 'complex real imag'");
                }
            }
            println!();
            continue 'main_loop;
        }

        // BITWISE OPERATIONS
        if expression.starts_with("AND ") || expression.starts_with("OR ") || expression.starts_with("XOR ") {
            let op = if expression.starts_with("AND ") { "AND" } else if expression.starts_with("OR ") { "OR" } else { "XOR" };
            temp_string = expression[4..].trim().to_string();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if tokens.len() == 2 {
                match (tokens[0].parse::<i64>(), tokens[1].parse::<i64>()) {
                    (Ok(a_val), Ok(b_val)) => {
                        bitwise_a = a_val;
                        bitwise_b = b_val;

                        if op == "AND" {
                            bitwise_result = bitwise_a & bitwise_b;
                        } else if op == "OR" {
                            bitwise_result = bitwise_a | bitwise_b;
                        } else {
                            bitwise_result = bitwise_a ^ bitwise_b;
                        }

                        result = bitwise_result as f64;
                        temp_vars.push(result);
                        if temp_vars.len() > 10 { temp_vars.remove(0); }

                        if use_colors {
                            println!("\x1b[32m{} {} {} = {} (0x{:X})\x1b[0m", bitwise_a, op, bitwise_b, bitwise_result, bitwise_result);
                        } else {
                            println!("{} {} {} = {} (0x{:X})", bitwise_a, op, bitwise_b, bitwise_result, bitwise_result);
                        }
                    }
                    _ => {
                        if use_colors {
                            println!("\x1b[31mError: Invalid integers\x1b[0m");
                        } else {
                            println!("Error: Invalid integers");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need two integers\x1b[0m");
                } else {
                    println!("Error: Need two integers");
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("NOT ") {
            temp_string = expression[4..].trim().to_string();
            match temp_string.parse::<i64>() {
                Ok(val) => {
                    bitwise_result = !val;
                    result = bitwise_result as f64;
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32mNOT {} = {} (0x{:X})\x1b[0m", val, bitwise_result, bitwise_result);
                    } else {
                        println!("NOT {} = {} (0x{:X})", val, bitwise_result, bitwise_result);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid integer\x1b[0m");
                    } else {
                        println!("Error: Invalid integer");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("SHL ") || expression.starts_with("SHR ") {
            let op = if expression.starts_with("SHL ") { "SHL" } else { "SHR" };
            temp_string = expression[4..].trim().to_string();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if tokens.len() == 2 {
                match (tokens[0].parse::<i64>(), tokens[1].parse::<u32>()) {
                    (Ok(val), Ok(shift)) => {
                        if op == "SHL" {
                            bitwise_result = val << shift;
                        } else {
                            bitwise_result = val >> shift;
                        }

                        result = bitwise_result as f64;
                        temp_vars.push(result);
                        if temp_vars.len() > 10 { temp_vars.remove(0); }

                        if use_colors {
                            println!("\x1b[32m{} {} {} = {} (0x{:X})\x1b[0m", val, op, shift, bitwise_result, bitwise_result);
                        } else {
                            println!("{} {} {} = {} (0x{:X})", val, op, shift, bitwise_result, bitwise_result);
                        }
                    }
                    _ => {
                        if use_colors {
                            println!("\x1b[31mError: Invalid values\x1b[0m");
                        } else {
                            println!("Error: Invalid values");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need value and shift amount\x1b[0m");
                } else {
                    println!("Error: Need value and shift amount");
                }
            }
            println!();
            continue 'main_loop;
        }

        // COMBINATIONS AND PERMUTATIONS
        if expression.starts_with("nCr ") || expression.starts_with("nPr ") {
            let op = if expression.starts_with("nCr ") { "nCr" } else { "nPr" };
            temp_string = expression[4..].trim().to_string();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if tokens.len() == 2 {
                match (tokens[0].parse::<u64>(), tokens[1].parse::<u64>()) {
                    (Ok(n), Ok(r)) => {
                        ncr_n = n;
                        ncr_r = r;

                        if ncr_r > ncr_n {
                            if use_colors {
                                println!("\x1b[31mError: r cannot be greater than n\x1b[0m");
                            } else {
                                println!("Error: r cannot be greater than n");
                            }
                        } else {
                            // Calculate nPr first
                            npr_result = 1;
                            i = 0;
                            while i < ncr_r as usize {
                                npr_result *= ncr_n - i as u64;
                                i += 1;
                            }

                            if op == "nPr" {
                                result = npr_result as f64;
                                temp_vars.push(result);
                                if temp_vars.len() > 10 { temp_vars.remove(0); }
                                if use_colors {
                                    println!("\x1b[32mnPr({}, {}) = {}\x1b[0m", ncr_n, ncr_r, npr_result);
                                } else {
                                    println!("nPr({}, {}) = {}", ncr_n, ncr_r, npr_result);
                                }
                            } else {
                                // Calculate r!
                                factorial_result = 1;
                                i = 1;
                                while i <= ncr_r as usize {
                                    factorial_result *= i as u64;
                                    i += 1;
                                }

                                ncr_result = npr_result / factorial_result;
                                result = ncr_result as f64;
                                temp_vars.push(result);
                                if temp_vars.len() > 10 { temp_vars.remove(0); }
                                if use_colors {
                                    println!("\x1b[32mnCr({}, {}) = {}\x1b[0m", ncr_n, ncr_r, ncr_result);
                                } else {
                                    println!("nCr({}, {}) = {}", ncr_n, ncr_r, ncr_result);
                                }
                            }
                        }
                    }
                    _ => {
                        if use_colors {
                            println!("\x1b[31mError: Invalid integers\x1b[0m");
                        } else {
                            println!("Error: Invalid integers");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need n and r values\x1b[0m");
                } else {
                    println!("Error: Need n and r values");
                }
            }
            println!();
            continue 'main_loop;
        }

        // ANGLE CONVERSION
        if expression.starts_with("deg2rad ") {
            temp_string = expression[8..].trim().to_string();
            match temp_string.parse::<f64>() {
                Ok(degrees) => {
                    angle_value = degrees * std::f64::consts::PI / 180.0;
                    result = angle_value;
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32m{:.precision$}° = {:.precision$} radians\x1b[0m", degrees, angle_value, precision = precision);
                    } else {
                        println!("{:.precision$}° = {:.precision$} radians", degrees, angle_value, precision = precision);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid angle\x1b[0m");
                    } else {
                        println!("Error: Invalid angle");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("rad2deg ") {
            temp_string = expression[8..].trim().to_string();
            match temp_string.parse::<f64>() {
                Ok(radians) => {
                    angle_value = radians * 180.0 / std::f64::consts::PI;
                    result = angle_value;
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32m{:.precision$} radians = {:.precision$}°\x1b[0m", radians, angle_value, precision = precision);
                    } else {
                        println!("{:.precision$} radians = {:.precision$}°", radians, angle_value, precision = precision);
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid angle\x1b[0m");
                    } else {
                        println!("Error: Invalid angle");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        // QUADRATIC EQUATION SOLVER
        if expression.starts_with("quad ") {
            temp_string = expression[5..].trim().to_string();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if tokens.len() == 3 {
                match (tokens[0].parse::<f64>(), tokens[1].parse::<f64>(), tokens[2].parse::<f64>()) {
                    (Ok(a_val), Ok(b_val), Ok(c_val)) => {
                        quad_a = a_val;
                        quad_b = b_val;
                        quad_c = c_val;

                        if quad_a == 0.0 {
                            if use_colors {
                                println!("\x1b[31mError: Not a quadratic equation (a = 0)\x1b[0m");
                            } else {
                                println!("Error: Not a quadratic equation (a = 0)");
                            }
                        } else {
                            quad_discriminant = quad_b * quad_b - 4.0 * quad_a * quad_c;

                            if quad_discriminant < 0.0 {
                                if use_colors {
                                    println!("\x1b[33mNo real solutions (discriminant < 0)\x1b[0m");
                                } else {
                                    println!("No real solutions (discriminant < 0)");
                                }
                            } else if quad_discriminant == 0.0 {
                                quad_x1 = -quad_b / (2.0 * quad_a);
                                result = quad_x1;
                                temp_vars.push(result);
                                if temp_vars.len() > 10 { temp_vars.remove(0); }
                                if use_colors {
                                    println!("\x1b[32mOne solution: x = {:.precision$}\x1b[0m", quad_x1, precision = precision);
                                } else {
                                    println!("One solution: x = {:.precision$}", quad_x1, precision = precision);
                                }
                            } else {
                                quad_x1 = (-quad_b + quad_discriminant.sqrt()) / (2.0 * quad_a);
                                quad_x2 = (-quad_b - quad_discriminant.sqrt()) / (2.0 * quad_a);
                                result = quad_x1;
                                temp_vars.push(quad_x1);
                                temp_vars.push(quad_x2);
                                if temp_vars.len() > 10 { temp_vars.remove(0); }
                                if temp_vars.len() > 10 { temp_vars.remove(0); }
                                if use_colors {
                                    println!("\x1b[32mTwo solutions: x1 = {:.precision$}, x2 = {:.precision$}\x1b[0m", quad_x1, quad_x2, precision = precision);
                                } else {
                                    println!("Two solutions: x1 = {:.precision$}, x2 = {:.precision$}", quad_x1, quad_x2, precision = precision);
                                }
                            }
                        }
                    }
                    _ => {
                        if use_colors {
                            println!("\x1b[31mError: Invalid coefficients\x1b[0m");
                        } else {
                            println!("Error: Invalid coefficients");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need three coefficients (a b c)\x1b[0m");
                } else {
                    println!("Error: Need three coefficients (a b c)");
                }
            }
            println!();
            continue 'main_loop;
        }

        // LINEAR EQUATION SOLVER (simplified: ax + b = c)
        if expression.starts_with("solve ") {
            temp_string = expression[6..].trim().to_string();

            // Simple parser for "ax + b = c" format
            if temp_string.contains('=') {
                parts.clear();
                let mut current_part = String::new();
                chars_vec = temp_string.chars().collect();
                i = 0;
                while i < chars_vec.len() {
                    if chars_vec[i] == '=' {
                        parts.push(current_part.trim().to_string());
                        current_part.clear();
                    } else {
                        current_part.push(chars_vec[i]);
                    }
                    i += 1;
                }
                parts.push(current_part.trim().to_string());

                if parts.len() == 2 {
                    // Parse right side (c)
                    match parts[1].parse::<f64>() {
                        Ok(c_val) => {
                            linear_c = c_val;

                            // Parse left side (ax + b)
                            let left_side = &parts[0];

                            // Simple parsing for "2x + 3" format
                            if left_side.contains("x") {
                                // Extract coefficient of x and constant
                                let mut a_found = false;
                                linear_a = 0.0;
                                linear_b = 0.0;

                                // Very simple parser - assumes format like "2x + 3" or "x + 5"
                                if left_side.starts_with("x") {
                                    linear_a = 1.0;
                                    a_found = true;
                                } else {
                                    // Try to extract coefficient
                                    let x_pos = left_side.find('x');
                                    if let Some(pos) = x_pos {
                                        let coeff_str = &left_side[..pos];
                                        if !coeff_str.is_empty() {
                                            match coeff_str.parse::<f64>() {
                                                Ok(coeff) => {
                                                    linear_a = coeff;
                                                    a_found = true;
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                    }
                                }

                                // Extract constant term
                                if left_side.contains('+') {
                                    let plus_pos = left_side.rfind('+');
                                    if let Some(pos) = plus_pos {
                                        let const_str = &left_side[pos + 1..].trim();
                                        match const_str.parse::<f64>() {
                                            Ok(const_val) => {
                                                linear_b = const_val;
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                } else if left_side.contains('-') {
                                    let minus_pos = left_side.rfind('-');
                                    if let Some(pos) = minus_pos {
                                        if pos > 0 { // Not the first character
                                            let const_str = &left_side[pos + 1..].trim();
                                            match const_str.parse::<f64>() {
                                                Ok(const_val) => {
                                                    linear_b = -const_val;
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                    }
                                }

                                if a_found {
                                    if linear_a == 0.0 {
                                        if use_colors {
                                            println!("\x1b[31mError: No solution (coefficient of x is 0)\x1b[0m");
                                        } else {
                                            println!("Error: No solution (coefficient of x is 0)");
                                        }
                                    } else {
                                        linear_x = (linear_c - linear_b) / linear_a;
                                        result = linear_x;
                                        temp_vars.push(result);
                                        if temp_vars.len() > 10 { temp_vars.remove(0); }
                                        if use_colors {
                                            println!("\x1b[32mSolution: x = {:.precision$}\x1b[0m", linear_x, precision = precision);
                                        } else {
                                            println!("Solution: x = {:.precision$}", linear_x, precision = precision);
                                        }
                                    }
                                } else {
                                    if use_colors {
                                        println!("\x1b[31mError: Could not parse equation\x1b[0m");
                                    } else {
                                        println!("Error: Could not parse equation");
                                    }
                                }
                            } else {
                                if use_colors {
                                    println!("\x1b[31mError: No variable 'x' found\x1b[0m");
                                } else {
                                    println!("Error: No variable 'x' found");
                                }
                            }
                        }
                        Err(_) => {
                            if use_colors {
                                println!("\x1b[31mError: Invalid right side of equation\x1b[0m");
                            } else {
                                println!("Error: Invalid right side of equation");
                            }
                        }
                    }
                } else {
                    if use_colors {
                        println!("\x1b[31mError: Invalid equation format\x1b[0m");
                    } else {
                        println!("Error: Invalid equation format");
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Equation must contain '='\x1b[0m");
                } else {
                    println!("Error: Equation must contain '='");
                }
            }
            println!();
            continue 'main_loop;
        }

        // STATISTICS FUNCTIONS
        if expression.starts_with("mean ") {
            temp_string = expression[5..].trim().to_string();
            stats_values.clear();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if !tokens.is_empty() {
                has_error = false;
                for token in &tokens {
                    match token.parse::<f64>() {
                        Ok(val) => stats_values.push(val),
                        Err(_) => {
                            has_error = true;
                            break;
                        }
                    }
                }

                if !has_error {
                    mean_sum = 0.0;
                    for val in &stats_values {
                        mean_sum += val;
                    }
                    result = mean_sum / stats_values.len() as f64;
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32mMean = {:.precision$}\x1b[0m", result, precision = precision);
                    } else {
                        println!("Mean = {:.precision$}", result, precision = precision);
                    }
                } else {
                    if use_colors {
                        println!("\x1b[31mError: Invalid numbers\x1b[0m");
                    } else {
                        println!("Error: Invalid numbers");
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need at least one number\x1b[0m");
                } else {
                    println!("Error: Need at least one number");
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("sum ") {
            temp_string = expression[4..].trim().to_string();
            stats_values.clear();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if !tokens.is_empty() {
                has_error = false;
                for token in &tokens {
                    match token.parse::<f64>() {
                        Ok(val) => stats_values.push(val),
                        Err(_) => {
                            has_error = true;
                            break;
                        }
                    }
                }

                if !has_error {
                    result = 0.0;
                    for val in &stats_values {
                        result += val;
                    }
                    temp_vars.push(result);
                    if temp_vars.len() > 10 { temp_vars.remove(0); }
                    if use_colors {
                        println!("\x1b[32mSum = {:.precision$}\x1b[0m", result, precision = precision);
                    } else {
                        println!("Sum = {:.precision$}", result, precision = precision);
                    }
                } else {
                    if use_colors {
                        println!("\x1b[31mError: Invalid numbers\x1b[0m");
                    } else {
                        println!("Error: Invalid numbers");
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need at least one number\x1b[0m");
                } else {
                    println!("Error: Need at least one number");
                }
            }
            println!();
            continue 'main_loop;
        }

        // FRACTION OPERATIONS
        if expression.starts_with("frac ") {
            temp_string = expression[5..].trim().to_string();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if tokens.len() == 2 {
                match (tokens[0].parse::<i64>(), tokens[1].parse::<i64>()) {
                    (Ok(num), Ok(den)) => {
                        if den != 0 {
                            fraction_num = num;
                            fraction_den = den;

                            // Simplify fraction using GCD
                            gcd_a = fraction_num.abs() as u64;
                            gcd_b = fraction_den.abs() as u64;

                            // Calculate GCD
                            while gcd_b != 0 {
                                gcd_temp = gcd_b;
                                gcd_b = gcd_a % gcd_b;
                                gcd_a = gcd_temp;
                            }

                            fraction_num /= gcd_a as i64;
                            fraction_den /= gcd_a as i64;

                            // Make denominator positive
                            if fraction_den < 0 {
                                fraction_num = -fraction_num;
                                fraction_den = -fraction_den;
                            }

                            result = fraction_num as f64 / fraction_den as f64;
                            temp_vars.push(result);
                            if temp_vars.len() > 10 { temp_vars.remove(0); }

                            if use_colors {
                                println!("\x1b[32mFraction: {}/{} = {:.precision$}\x1b[0m", fraction_num, fraction_den, result, precision = precision);
                            } else {
                                println!("Fraction: {}/{} = {:.precision$}", fraction_num, fraction_den, result, precision = precision);
                            }
                        } else {
                            if use_colors {
                                println!("\x1b[31mError: Division by zero\x1b[0m");
                            } else {
                                println!("Error: Division by zero");
                            }
                        }
                    }
                    _ => {
                        if use_colors {
                            println!("\x1b[31mError: Invalid integers\x1b[0m");
                        } else {
                            println!("Error: Invalid integers");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need numerator and denominator\x1b[0m");
                } else {
                    println!("Error: Need numerator and denominator");
                }
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("gcd ") {
            temp_string = expression[4..].trim().to_string();

            tokens.clear();
            current_token.clear();
            chars_vec = temp_string.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
            }

            if tokens.len() == 2 {
                match (tokens[0].parse::<u64>(), tokens[1].parse::<u64>()) {
                    (Ok(a_val), Ok(b_val)) => {
                        gcd_a = a_val;
                        gcd_b = b_val;

                        // Calculate GCD using Euclidean algorithm
                        while gcd_b != 0 {
                            gcd_temp = gcd_b;
                            gcd_b = gcd_a % gcd_b;
                            gcd_a = gcd_temp;
                        }

                        result = gcd_a as f64;
                        temp_vars.push(result);
                        if temp_vars.len() > 10 { temp_vars.remove(0); }

                        if use_colors {
                            println!("\x1b[32mgcd({}, {}) = {}\x1b[0m", a_val, b_val, gcd_a);
                        } else {
                            println!("gcd({}, {}) = {}", a_val, b_val, gcd_a);
                        }
                    }
                    _ => {
                        if use_colors {
                            println!("\x1b[31mError: Invalid integers\x1b[0m");
                        } else {
                            println!("Error: Invalid integers");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Need two integers\x1b[0m");
                } else {
                    println!("Error: Need two integers");
                }
            }
            println!();
            continue 'main_loop;
        }

        // RANDOM AND PRIME FUNCTIONS
        if expression == "rand" {
            random_seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;

            // Simple linear congruential generator
            random_seed = (random_seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
            random_result = (random_seed % 1000) as f64 / 1000.0;

            result = random_result;
            temp_vars.push(result);
            if temp_vars.len() > 10 { temp_vars.remove(0); }

            if use_colors {
                println!("\x1b[32mRandom: {:.precision$}\x1b[0m", result, precision = precision);
            } else {
                println!("Random: {:.precision$}", result, precision = precision);
            }
            println!();
            continue 'main_loop;
        }

        if expression.starts_with("isprime ") {
            temp_string = expression[8..].trim().to_string();
            match temp_string.parse::<u64>() {
                Ok(val) => {
                    prime_check = val;
                    is_prime = true;

                    if prime_check < 2 {
                        is_prime = false;
                    } else if prime_check == 2 {
                        is_prime = true;
                    } else if prime_check % 2 == 0 {
                        is_prime = false;
                    } else {
                        i = 3;
                        while (i as u64) * (i as u64) <= prime_check {
                            if prime_check % (i as u64) == 0 {
                                is_prime = false;
                                break;
                            }
                            i += 2;
                        }
                    }

                    if use_colors {
                        if is_prime {
                            println!("\x1b[32m{} is prime\x1b[0m", prime_check);
                        } else {
                            println!("\x1b[33m{} is not prime\x1b[0m", prime_check);
                        }
                    } else {
                        if is_prime {
                            println!("{} is prime", prime_check);
                        } else {
                            println!("{} is not prime", prime_check);
                        }
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Invalid number\x1b[0m");
                    } else {
                        println!("Error: Invalid number");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        // Continue with existing commands (save, load, etc.)
        if expression == "save" {
            // Save variables and memory slots
            match fs::File::create("calculator_variables.txt") {
                Ok(mut file) => {
                    for (name, value) in &variables {
                        temp_string = format!("{} = {}\n", name, value);
                        let _ = file.write_all(temp_string.as_bytes());
                    }
                    for (name, value) in &memory_slots {
                        temp_string = format!("mem_{} = {}\n", name, value);
                        let _ = file.write_all(temp_string.as_bytes());
                    }
                    if use_colors {
                        println!("\x1b[32mSession saved\x1b[0m");
                    } else {
                        println!("Session saved");
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Could not save session\x1b[0m");
                    } else {
                        println!("Error: Could not save session");
                    }
                }
            }

            // Save aliases
            match fs::File::create("calculator_aliases.txt") {
                Ok(mut file) => {
                    for (name, expr) in &aliases {
                        temp_string = format!("{} = {}\n", name, expr);
                        let _ = file.write_all(temp_string.as_bytes());
                    }
                }
                Err(_) => {}
            }

            // Save settings
            match fs::File::create("calculator_settings.txt") {
                Ok(mut file) => {
                    for (name, value) in &settings {
                        temp_string = format!("{} = {}\n", name, value);
                        let _ = file.write_all(temp_string.as_bytes());
                    }
                }
                Err(_) => {}
            }

            println!();
            continue 'main_loop;
        }

        // Load command
        if expression == "load" {
            variables.clear();
            memory_slots.clear();
            aliases.clear();

            // Load variables and memory slots
            match fs::read_to_string("calculator_variables.txt") {
                Ok(content) => {
                    chars_vec = content.chars().collect();
                    temp_string.clear();
                    i = 0;
                    while i < chars_vec.len() {
                        if chars_vec[i] == '\n' {
                            if !temp_string.is_empty() {
                                if temp_string.contains('=') {
                                    parts.clear();
                                    let mut current_part = String::new();
                                    j = 0;
                                    while j < temp_string.len() {
                                        let ch = temp_string.chars().nth(j).unwrap();
                                        if ch == '=' {
                                            parts.push(current_part.trim().to_string());
                                            current_part.clear();
                                        } else {
                                            current_part.push(ch);
                                        }
                                        j += 1;
                                    }
                                    parts.push(current_part.trim().to_string());

                                    if parts.len() == 2 {
                                        if let Ok(val) = parts[1].parse::<f64>() {
                                            if parts[0].starts_with("mem_") {
                                                memory_slots.insert(parts[0][4..].to_string(), val);
                                            } else {
                                                variables.insert(parts[0].clone(), val);
                                            }
                                        }
                                    }
                                }
                                temp_string.clear();
                            }
                        } else {
                            temp_string.push(chars_vec[i]);
                        }
                        i += 1;
                    }

                    if use_colors {
                        println!("\x1b[32mSession loaded\x1b[0m");
                    } else {
                        println!("Session loaded");
                    }
                }
                Err(_) => {
                    if use_colors {
                        println!("\x1b[31mError: Could not load session\x1b[0m");
                    } else {
                        println!("Error: Could not load session");
                    }
                }
            }
            println!();
            continue 'main_loop;
        }

        // Settings command
        if expression == "settings" {
            if use_colors {
                println!("\x1b[36mCurrent Settings:\x1b[0m");
            } else {
                println!("Current Settings:");
            }
            for (name, value) in &settings {
                if use_colors {
                    println!("  \x1b[32m{}\x1b[0m = \x1b[33m{}\x1b[0m", name, value);
                } else {
                    println!("  {} = {}", name, value);
                }
            }
            println!();
            continue 'main_loop;
        }

        // History command
        if expression == "history" {
            if history.is_empty() {
                if use_colors {
                    println!("\x1b[33mNo history available.\x1b[0m");
                } else {
                    println!("No history available.");
                }
            } else {
                if use_colors {
                    println!("\x1b[36mHistory:\x1b[0m");
                } else {
                    println!("History:");
                }
                i = 0;
                while i < history.len() {
                    if use_colors {
                        println!("  \x1b[34m{}\x1b[0m: {}", i + 1, history[i]);
                    } else {
                        println!("  {}: {}", i + 1, history[i]);
                    }
                    i += 1;
                }
            }
            println!();
            continue 'main_loop;
        }

        // Dump command
        if expression == "dump" {
            if use_colors {
                println!("\x1b[36m=== Symbol Table Dump ===\x1b[0m");
                println!("\x1b[33mVariables:\x1b[0m");
            } else {
                println!("=== Symbol Table Dump ===");
                println!("Variables:");
            }
            for (name, value) in &variables {
                if use_colors {
                    println!("  \x1b[32m{}\x1b[0m = \x1b[33m{:.precision$}\x1b[0m", name, value, precision = precision);
                } else {
                    println!("  {} = {:.precision$}", name, value, precision = precision);
                }
            }

            if use_colors {
                println!("\x1b[33mMemory Slots:\x1b[0m");
            } else {
                println!("Memory Slots:");
            }
            for (name, value) in &memory_slots {
                if use_colors {
                    println!("  \x1b[32m{}\x1b[0m = \x1b[33m{:.precision$}\x1b[0m", name, value, precision = precision);
                } else {
                    println!("  {} = {:.precision$}", name, value, precision = precision);
                }
            }

            if use_colors {
                println!("\x1b[33mConstants:\x1b[0m");
            } else {
                println!("Constants:");
            }
            for (name, value) in &constants {
                if use_colors {
                    println!("  \x1b[32m{}\x1b[0m = \x1b[33m{:.precision$}\x1b[0m", name, value, precision = precision);
                } else {
                    println!("  {} = {:.precision$}", name, value, precision = precision);
                }
            }

            if use_colors {
                println!("\x1b[33mAliases:\x1b[0m");
            } else {
                println!("Aliases:");
            }
            for (name, expr) in &aliases {
                if use_colors {
                    println!("  \x1b[32m{}\x1b[0m = \x1b[33m{}\x1b[0m", name, expr);
                } else {
                    println!("  {} = {}", name, expr);
                }
            }
            println!();
            continue 'main_loop;
        }

        // MAIN EXPRESSION EVALUATION - COMPLETELY PROCEDURAL
        has_error = false;
        processed_expr = expression.clone();

        // Handle temporary variables ($1, $2, $3)
        if processed_expr.contains('$') {
            i = 1;
            while i <= temp_vars.len() && i <= 10 {
                temp_string = format!("${}", i);
                if processed_expr.contains(&temp_string) {
                    let temp_val = temp_vars[temp_vars.len() - i];
                    processed_expr = processed_expr.replace(&temp_string, &temp_val.to_string());
                }
                i += 1;
            }
        }

        // Remove comments
        if processed_expr.contains('#') {
            chars_vec = processed_expr.chars().collect();
            temp_string.clear();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == '#' {
                    break;
                }
                temp_string.push(chars_vec[i]);
                i += 1;
            }
            processed_expr = temp_string.trim().to_string();
        }

        if processed_expr.is_empty() {
            continue 'main_loop;
        }

        // Check for variable assignment
        if processed_expr.contains('=') && !processed_expr.contains("==") {
            parts.clear();
            let mut current_part = String::new();
            chars_vec = processed_expr.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == '=' {
                    parts.push(current_part.trim().to_string());
                    current_part.clear();
                } else {
                    current_part.push(chars_vec[i]);
                }
                i += 1;
            }
            parts.push(current_part.trim().to_string());

            if parts.len() == 2 {
                var_name = parts[0].clone();
                var_value = parts[1].clone();

                // Validate variable name
                has_error = false;
                chars_vec = var_name.chars().collect();
                if chars_vec.is_empty() || chars_vec[0].is_ascii_digit() {
                    has_error = true;
                } else {
                    i = 0;
                    while i < chars_vec.len() {
                        if !chars_vec[i].is_alphanumeric() && chars_vec[i] != '_' {
                            has_error = true;
                            break;
                        }
                        i += 1;
                    }
                }

                if has_error {
                    if use_colors {
                        println!("\x1b[31mError: Invalid variable name\x1b[0m");
                    } else {
                        println!("Error: Invalid variable name");
                    }
                } else {
                    // Evaluate the right side
                    processed_expr = var_value.clone();

                    // Replace aliases
                    for (alias_name, alias_expr) in &aliases {
                        processed_expr = processed_expr.replace(alias_name, alias_expr);
                    }

                    // Replace constants
                    for (const_name, const_value) in &constants {
                        processed_expr = processed_expr.replace(const_name, &const_value.to_string());
                    }

                    // Replace variables and memory slots
                    for (existing_var, existing_val) in &variables {
                        if existing_var != &var_name {
                            processed_expr = processed_expr.replace(existing_var, &existing_val.to_string());
                        }
                    }
                    for (mem_name, mem_val) in &memory_slots {
                        processed_expr = processed_expr.replace(mem_name, &mem_val.to_string());
                    }

                    // PROCEDURAL ARITHMETIC EVALUATION
                    has_error = false;
                    result = 0.0;

                    // Simple arithmetic evaluation
                    if processed_expr.contains('+') || processed_expr.contains('-') || processed_expr.contains('*') || processed_expr.contains('/') || processed_expr.contains('^') {
                        // Tokenize expression
                        tokens.clear();
                        current_token.clear();
                        chars_vec = processed_expr.chars().collect();
                        i = 0;
                        while i < chars_vec.len() {
                            if chars_vec[i] == '+' || chars_vec[i] == '-' || chars_vec[i] == '*' || chars_vec[i] == '/' || chars_vec[i] == '^' {
                                if !current_token.is_empty() {
                                    tokens.push(current_token.trim().to_string());
                                    current_token.clear();
                                }
                                tokens.push(chars_vec[i].to_string());
                            } else if chars_vec[i] == ' ' {
                                if !current_token.is_empty() {
                                    tokens.push(current_token.trim().to_string());
                                    current_token.clear();
                                }
                            } else {
                                current_token.push(chars_vec[i]);
                            }
                            i += 1;
                        }
                        if !current_token.is_empty() {
                            tokens.push(current_token.trim().to_string());
                        }

                        // Simple left-to-right evaluation with basic precedence
                        if tokens.len() >= 3 {
                            match tokens[0].parse::<f64>() {
                                Ok(first_val) => {
                                    result = first_val;
                                    i = 1;
                                    while i < tokens.len() - 1 {
                                        let operator = &tokens[i];
                                        match tokens[i + 1].parse::<f64>() {
                                            Ok(next_val) => {
                                                if operator == "+" {
                                                    result += next_val;
                                                } else if operator == "-" {
                                                    result -= next_val;
                                                } else if operator == "*" {
                                                    result *= next_val;
                                                } else if operator == "/" {
                                                    if next_val != 0.0 {
                                                        result /= next_val;
                                                    } else {
                                                        has_error = true;
                                                        break;
                                                    }
                                                } else if operator == "^" {
                                                    result = result.powf(next_val);
                                                }
                                            }
                                            Err(_) => {
                                                has_error = true;
                                                break;
                                            }
                                        }
                                        i += 2;
                                    }
                                }
                                Err(_) => {
                                    has_error = true;
                                }
                            }
                        } else {
                            has_error = true;
                        }
                    } else {
                        match processed_expr.parse::<f64>() {
                            Ok(val) => result = val,
                            Err(_) => has_error = true,
                        }
                    }

                    if !has_error {
                        variables.insert(var_name.clone(), result);

                        // Add to temporary variables
                        temp_vars.push(result);
                        if temp_vars.len() > 10 {
                            temp_vars.remove(0);
                        }

                        if use_colors {
                            println!("\x1b[32m{} = {:.precision$}\x1b[0m", var_name, result, precision = precision);
                        } else {
                            println!("{} = {:.precision$}", var_name, result, precision = precision);
                        }
                    } else {
                        if use_colors {
                            println!("\x1b[31mError: Could not evaluate expression\x1b[0m");
                        } else {
                            println!("Error: Could not evaluate expression");
                        }
                    }
                }
            } else {
                if use_colors {
                    println!("\x1b[31mError: Invalid assignment format\x1b[0m");
                } else {
                    println!("Error: Invalid assignment format");
                }
            }

            // Log to audit
            result_string = if has_error { "ERROR".to_string() } else { "OK".to_string() };
            audit_log = format!("{}: {} -> {}\n", timestamp, expression, result_string);
            let _ = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("calculator_audit.log")
                .and_then(|mut file| file.write_all(audit_log.as_bytes()));

            println!();
            continue 'main_loop;
        }

        // Replace aliases
        for (alias_name, alias_expr) in &aliases {
            processed_expr = processed_expr.replace(alias_name, alias_expr);
        }

        // Replace constants
        for (const_name, const_value) in &constants {
            processed_expr = processed_expr.replace(const_name, &const_value.to_string());
        }

        // Replace variables and memory slots
        for (var_name_ref, var_value_ref) in &variables {
            processed_expr = processed_expr.replace(var_name_ref, &var_value_ref.to_string());
        }
        for (mem_name, mem_val) in &memory_slots {
            processed_expr = processed_expr.replace(mem_name, &mem_val.to_string());
        }

        // PROCEDURAL ARITHMETIC EVALUATION
        has_error = false;
        result = 0.0;

        // Simple arithmetic evaluation
        if processed_expr.contains('+') || processed_expr.contains('-') || processed_expr.contains('*') || processed_expr.contains('/') || processed_expr.contains('^') || processed_expr.contains('%') {
            // Tokenize expression
            tokens.clear();
            current_token.clear();
            chars_vec = processed_expr.chars().collect();
            i = 0;
            while i < chars_vec.len() {
                if chars_vec[i] == '+' || chars_vec[i] == '-' || chars_vec[i] == '*' || chars_vec[i] == '/' || chars_vec[i] == '^' || chars_vec[i] == '%' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(chars_vec[i].to_string());
                } else if chars_vec[i] == ' ' {
                    if !current_token.is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                } else {
                    current_token.push(chars_vec[i]);
                }
                i += 1;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.trim().to_string());
            }

            // Simple left-to-right evaluation with basic precedence
            if tokens.len() >= 3 {
                match tokens[0].parse::<f64>() {
                    Ok(first_val) => {
                        result = first_val;
                        i = 1;
                        while i < tokens.len() - 1 {
                            let operator = &tokens[i];
                            match tokens[i + 1].parse::<f64>() {
                                Ok(next_val) => {
                                    if operator == "+" {
                                        result += next_val;
                                    } else if operator == "-" {
                                        result -= next_val;
                                    } else if operator == "*" {
                                        result *= next_val;
                                    } else if operator == "/" {
                                        if next_val != 0.0 {
                                            result /= next_val;
                                        } else {
                                            has_error = true;
                                            break;
                                        }
                                    } else if operator == "^" {
                                        result = result.powf(next_val);
                                    } else if operator == "%" {
                                        result = result % next_val;
                                    }
                                }
                                Err(_) => {
                                    has_error = true;
                                    break;
                                }
                            }
                            i += 2;
                        }
                    }
                    Err(_) => {
                        has_error = true;
                    }
                }
            } else {
                has_error = true;
            }
        } else {
            match processed_expr.parse::<f64>() {
                Ok(val) => result = val,
                Err(_) => has_error = true,
            }
        }

        if !has_error {
            // Add to temporary variables
            temp_vars.push(result);
            if temp_vars.len() > 10 {
                temp_vars.remove(0);
            }

            if use_colors {
                println!("\x1b[32mResult: {:.precision$}\x1b[0m", result, precision = precision);
                if timer_enabled {
                    elapsed_time = start_time.elapsed();
                    println!("\x1b[36mTime: {:.3}ms\x1b[0m", elapsed_time.as_secs_f64() * 1000.0);
                }
            } else {
                println!("Result: {:.precision$}", result, precision = precision);
                if timer_enabled {
                    elapsed_time = start_time.elapsed();
                    println!("Time: {:.3}ms", elapsed_time.as_secs_f64() * 1000.0);
                }
            }
        } else {
            if use_colors {
                println!("\x1b[31mError: Invalid expression\x1b[0m");
            } else {
                println!("Error: Invalid expression");
            }
        }

        // Log to audit
        result_string = if has_error {
            "ERROR".to_string()
        } else {
            format!("{:.precision$}", result, precision = precision)
        };
        audit_log = format!("{}: {} -> {}\n", timestamp, expression, result_string);
        let _ = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("calculator_audit.log")
            .and_then(|mut file| file.write_all(audit_log.as_bytes()));

        // Check if script mode is done
        if script_mode && script_index >= script_lines.len() {
            script_mode = false;
            if use_colors {
                println!("\x1b[32mScript execution completed.\x1b[0m");
            } else {
                println!("Script execution completed.");
            }
        }

        // Process remaining commands in chain
        if command_chain.len() > 1 && chain_index < command_chain.len() {
            expression = command_chain[chain_index].clone();
            chain_index += 1;
            if use_colors {
                println!("\x1b[35m[Chain {}] {}\x1b[0m", chain_index, expression);
            } else {
                println!("[Chain {}] {}", chain_index, expression);
            }
            // Continue to process this command
            continue 'main_loop;
        }

        println!();
    }

    // AUTO-BACKUP ON EXIT
    if settings.get("auto_backup").unwrap_or(&"true".to_string()) == "true" {
        // Create backup with timestamp
        let backup_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let backup_filename = format!("calculator_backup_{}.txt", backup_timestamp);

        match fs::File::create(&backup_filename) {
            Ok(mut file) => {
                temp_string = format!("# Calculator Backup - {}\n", backup_timestamp);
                let _ = file.write_all(temp_string.as_bytes());

                temp_string = "# Variables\n".to_string();
                let _ = file.write_all(temp_string.as_bytes());
                for (name, value) in &variables {
                    temp_string = format!("{} = {}\n", name, value);
                    let _ = file.write_all(temp_string.as_bytes());
                }

                temp_string = "# Memory Slots\n".to_string();
                let _ = file.write_all(temp_string.as_bytes());
                for (name, value) in &memory_slots {
                    temp_string = format!("mem_{} = {}\n", name, value);
                    let _ = file.write_all(temp_string.as_bytes());
                }

                temp_string = "# Aliases\n".to_string();
                let _ = file.write_all(temp_string.as_bytes());
                for (name, expr) in &aliases {
                    temp_string = format!("alias {} = {}\n", name, expr);
                    let _ = file.write_all(temp_string.as_bytes());
                }

                if use_colors {
                    println!("\x1b[36mAuto-backup saved to {}\x1b[0m", backup_filename);
                } else {
                    println!("Auto-backup saved to {}", backup_filename);
                }
            }
            Err(_) => {}
        }
    }

    // Save current session
    match fs::File::create("calculator_variables.txt") {
        Ok(mut file) => {
            for (name, value) in &variables {
                temp_string = format!("{} = {}\n", name, value);
                let _ = file.write_all(temp_string.as_bytes());
            }
            for (name, value) in &memory_slots {
                temp_string = format!("mem_{} = {}\n", name, value);
                let _ = file.write_all(temp_string.as_bytes());
            }
        }
        Err(_) => {}
    }

    match fs::File::create("calculator_aliases.txt") {
        Ok(mut file) => {
            for (name, expr) in &aliases {
                temp_string = format!("{} = {}\n", name, expr);
                let _ = file.write_all(temp_string.as_bytes());
            }
        }
        Err(_) => {}
    }

    match fs::File::create("calculator_settings.txt") {
        Ok(mut file) => {
            for (name, value) in &settings {
                temp_string = format!("{} = {}\n", name, value);
                let _ = file.write_all(temp_string.as_bytes());
            }
        }
        Err(_) => {}
    }

    if use_colors {
        println!("\x1b[36mGoodbye! Session saved with auto-backup.\x1b[0m");
    } else {
        println!("Goodbye! Session saved with auto-backup.");
    }
}
