use std::io;

fn main() {
    println!("Poketwo Pokedex");
    println!("");
    println!("A tool for finding a pokemon based on poketwo's obfuscated name");
    println!("================");

    let all_pokemon = include_str!("pokemon.txt");
    let all_pokemon: Vec<String> = all_pokemon
        .split("\r\n")
        .map(|pokemon| pokemon.to_lowercase())
        .collect();
    let all_pokemon_str: Vec<&str> = all_pokemon.iter().map(|pokemon| pokemon.as_str()).collect();

    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.trim();
        if command.starts_with(":") {
            let command = command.trim_start_matches(":");
            match command {
                "exit" | "q" | "quit" => break,
                "list" => list_pokemon(&all_pokemon_str),
                "help" => help(),
                _ => println!("Unknown command"),
            }
        } else {
            let guess = command.to_lowercase();
            let matched_pokemon: Vec<&str> = all_pokemon_str
                .iter()
                .filter(|pokemon| {
                    if pokemon.len() != guess.len() {
                        return false;
                    }
                    for (guess_char, pokemon_char) in guess.chars().zip(pokemon.chars()) {
                        if guess_char == '_' {
                            continue;
                        } else if guess_char != pokemon_char {
                            return false;
                        }
                    }
                    true
                })
                .copied()
                .collect();
            list_pokemon(&matched_pokemon);
        }
    }
}

fn list_pokemon(pokemon_vec: &Vec<&str>) {
    if pokemon_vec.len() != 0 {
        println!("Found {} pokemon:", pokemon_vec.len());
        for pokemon in &pokemon_vec[..] {
            println!("> {}", pokemon);
        }
    } else {
        println!("0 Pokemon found");
    }
}

fn help() {
    println!("Available commands:");
    println!("> list");
    println!("> help");
    println!("> exit");
}
