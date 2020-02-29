use std::env;
use std::io::ErrorKind::*;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn help() {
    println!("usage: cargo run <file> [file] [file] ...");
}

fn devax(input: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let mut clean = Vec::new();

    let mut mode: u8 = 0;
    let mut num: usize = 0;
    let zero: u8 = 0;

    for i in input.iter(){
        if i == &zero {
            mode = (mode + 1) % 2;
            continue;
        }

        if mode == 0 {
            num += *i as usize;
            println!("grabbing chunk of {}", i);
        } else if mode == 1 {
            clean.push(*i);
        }
    }
    if clean.len() == num {
        println!("...file OK!");
    } else {
        println!("...something went wrong!");
    }

    Ok(clean)
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

        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf)?;

        let xor = devax(buf)?;

/*
        for i in 0..buf.len() {
            xor.push(buf[i] ^ 0xff);
        }
*/
        out.write_all(&xor)?;
        out.flush()?;

        println!("Success!");
    }
    Ok(())
}
