use std::env;
use std::io::ErrorKind::*;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn help() {
    println!("usage: cargo run <file> [file] [file] ...");
}

fn devax(input: Vec<u8>) -> Vec<u8> {
    let mut clean = Vec::new();
    let mut mode: u8 = 0;
    let mut num: usize = 0;
    let mut size: u8 = 0;
    let zero: u8 = 0;

    for i in input.iter(){
        if i == &zero {
            continue;
        } else if mode == 0 {
            num += *i as usize;
            size = *i;
            mode = 1;
        } else if mode == 1 {
            clean.push(*i);
            if size > 0 {size -= 1};
            if size == zero {
                mode = 0;
                clean.push(b'\n');
                num += 1;
            }
        }
    }
    if clean.len() == num {
        println!("file OK!");
    } else {
        println!("something went wrong!");
        clean.clear();
        clean.truncate(0);
    }

    clean
}
fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    let mut input: Vec<String> = Vec::new();

    if args.len() < 2 {
        help();
        std::process::exit(1);
    }

    while args.len() > 1 {
        input.push(args.pop().unwrap());
    }

    while input.len() > 0 {
        let filename = input.pop().unwrap();
        print!("Cleaning up {}...", filename);

        let mut output = String::new();
        output.push_str(&filename);
        output.push_str(".clean");

        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(e)   => {
                match e.kind() {
                    NotFound => println!("Error! File not found."),
                    k        => println!("Error! {:?}", k)
                }
                continue;
            }
        };

        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf)?;

        /* run the de-vax-ify function */
        let xor = devax(buf);

        /* write out the de-vax-ified vector to file */
        if xor.is_empty() != true {
            let mut out = match File::create(&output) {
                Ok(out) => out,
                Err(e)  => {
                    match e.kind() {
                        PermissionDenied => println!("Error! Permission denied."),
                        k                => println!("Error! {:?}", k)
                    }
                    continue;
                }
            };

            out.write_all(&xor)?;
            out.flush()?;
        }
    }
    Ok(())
}
