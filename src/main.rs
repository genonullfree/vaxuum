use std::env;
use std::io::ErrorKind::*;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn help() {
    println!("usage: cargo run <file> [file] [file] ...");
}

fn devax(input: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let clean = Vec::new();

    for i in input.iter(){
        println!("{}", i);
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
        let mut xor: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf)?;

        let mut xor = devax(buf)?;

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
