use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file> <add|del>", args[0]);
        std::process::exit(1);
    }

    let file = &args[1];
    let flag = &args[2];

    let data = fs::read(file)?;
    let mut new_data = Vec::new();

    match flag.as_str() {
        "add" => {
            println!("adding 1 as header");
            new_data.push(1);
            new_data.extend(data);
        }
        "del" => {
            if data[0] == 1 {
                println!("removing 1 as header");
                new_data.extend(&data[1..]);
            } else {
                new_data.extend(data);
            }
        }
        _ => {
            eprintln!("Invalid flag. Use add or del.");
            std::process::exit(1);
        }
    }

    let new_file = format!("{}_{}", file, flag);
    fs::write(new_file, new_data)?;

    Ok(())
}
