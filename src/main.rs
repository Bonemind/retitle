use clap::Parser;
use std::fs;
use std::fs::read_to_string;
use std::fs::rename;
use std::fs::write;
use std::io::Read;

struct RenamePair {
    from: String,
    to: String,
}

impl Clone for RenamePair {
    fn clone(&self) -> Self {
        RenamePair {
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}

/// Retitle, simple tool to bulk rename files in the current directory.
/// By default it'll open a text editor to edit the files to their new names.
/// Alternate options are to read and write the list of operations from stdin/out or file
/// A retitle rename line is <original_name>|<new_name>. One per line
#[derive(Parser, Debug)]
#[group(required = false, multiple = false)]
struct CliArgs {
    /// Output to stdout
    #[arg(short = 'o', long)]
    stdout: bool,

    /// Input from stdin
    #[arg(short = 'i', long)]
    stdin: bool,

    /// Exports to file
    #[arg(short = 'e', long)]
    file_out: Option<String>,

    /// Resume from file
    #[arg(short = 'r', long)]
    file_in: Option<String>,
}

fn process_renames(rename_pairs: Vec<RenamePair>) {
    let mut had_error = false;
    let mut procesed: Vec<RenamePair> = vec![];
    for pair in rename_pairs.iter() {
        if &pair.from == &pair.to {
            continue;
        }
        let result = rename(&pair.from, &pair.to);
        match result {
            Ok(_) => {
                println!("Renamed {} to {}", pair.from, pair.to);
                procesed.push(pair.clone());
            }
            Err(e) => {
                println!("Failed to rename {} to {}: {}", pair.from, pair.to, e);
                had_error = true;
                break;
            }
        }
    }

    if !had_error {
        return;
    }
    println!("Rolling back renames due to errors");
    for pair in procesed.iter().rev() {
        let result = rename(&pair.to, &pair.from);
        match result {
            Ok(_) => {
                println!("Rolled back {} to {}", pair.to, pair.from);
            }
            Err(e) => {
                panic!("Failed to rename {} to {}: {}", pair.to, pair.from, e);
            }
        }
    }
}

fn format_renamepairs(rename_pairs: Vec<RenamePair>) -> String {
    let mut result = String::new();
    rename_pairs.iter().for_each(|pair| {
        result.push_str(&format!("{}|{}\n", pair.from, pair.to));
    });
    return result;
}

fn parse_renamepairs(input: String) -> Vec<RenamePair> {
    let pairs = input
        .lines()
        .map(|line| {
            let spl = line.split("|").collect::<Vec<&str>>();
            if spl.len() != 2 {
                panic!(
                    "Invalid input. Every line should be <from>|<to>, got {}",
                    line
                );
            }
            return RenamePair {
                from: spl[0].to_string(),
                to: spl[1].to_string(),
            };
        })
        .collect();
    return pairs;
}

fn get_file_list() -> Vec<String> {
    let listing = fs::read_dir("./").unwrap();

    let entries = listing
        .map(|p| match p {
            Err(e) => {
                println!("{}", e);
                return None;
            }
            Ok(entry) => {
                return Some(entry);
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap());

    let file_names: Vec<String> = entries
        .map(|x| x.file_name())
        .map(|x| x.into_string().unwrap())
        .collect();
    return file_names;
}

fn file_list_to_renamepairs() -> Vec<RenamePair> {
    let file_names = get_file_list();
    let rename_pairs = file_names
        .iter()
        .map(|x| RenamePair {
            from: x.to_string(),
            to: x.to_string(),
        })
        .collect();
    return rename_pairs;
}

fn main() {
    let cli_args = CliArgs::parse();

    if let Some(in_file) = &cli_args.file_in {
        let result = read_to_string(in_file);
        match result {
            Ok(read) => {
                let rename_pairs = parse_renamepairs(read);
                process_renames(rename_pairs);
            }
            Err(e) => {
                panic!("Failed to read from file: {}", e);
            }
        }
    } else if let Some(out_file) = &cli_args.file_out {
        let rename_pairs = file_list_to_renamepairs();
        let formatted = format_renamepairs(rename_pairs);
        let result = write(out_file, formatted);
        if let Err(e) = result {
            panic!("Failed to write to file: {}", e);
        }
    } else if cli_args.stdin {
        let mut stdin = std::io::stdin();
        let mut buf = String::new();

        let result = stdin.read_to_string(&mut buf);
        if let Err(e) = result {
            panic!("Failed to read from stdin: {}", e);
        }

        let rename_pairs = parse_renamepairs(buf);
        process_renames(rename_pairs);
    } else if cli_args.stdout {
        let rename_pairs = file_list_to_renamepairs();
        print!("{}", format_renamepairs(rename_pairs));
    } else {
        let rename_pairs = file_list_to_renamepairs();
        let formatted = format_renamepairs(rename_pairs);
        let edited = edit::edit(&formatted);
        match edited {
            Ok(edited) => {
                let parsed_pairs = parse_renamepairs(edited);
                process_renames(parsed_pairs);
            }
            Err(e) => {
                panic!("Failed to edit: {}", e);
            }
        }
    }
}
