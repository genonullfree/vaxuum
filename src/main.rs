use std::env;
use std::fs::File;
use std::io::ErrorKind::*;
use std::io::Read;
use std::io::Write;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct VaxError {
    details: String,
}

impl VaxError {
    fn new(msg: &str) -> VaxError {
        VaxError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for VaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for VaxError {
    fn description(&self) -> &str {
        &self.details
    }
}

/* print the usage */
fn help() {
    println!("usage: cargo run <file> [file] [file] ...");
}

/* function to read in the original file and generate the correct file output */
fn devax(input: Vec<u8>) -> Result<Vec<u8>, VaxError> {
    let mut clean = Vec::new();
    let mut mode: u8 = 0;
    let mut num: usize = 0;
    let mut size: u8 = 0;
    let zero: u8 = 0;

    /* iterate through each byte of the original file */
    for i in input.iter(){
        /* if a null is encountered, ignore */
        /* TODO: fix this to handle 16bit little endian values */
        if i == &zero {
            continue;
        }
        /* if the mode is 0, treat as reading in a single number */
        else if mode == 0 {
            num += *i as usize;
            size = *i;
            mode = 1;
        }
        /* if the mode is 1, treat as reading in that number of bytes */
        else if mode == 1 {
            clean.push(*i);

            /* decrement size of byte-munching */
            if size > 0 {size -= 1};

            /* if size gets decremented to 0, push a newline and increment total
                number of bytes */
            if size == zero {
                mode = 0;
                clean.push(b'\n');
                num += 1;
            }
        }
    }

    /* if the bytes written equal the same number of bytes that we believe should
        be written, then the vaxuum-ing was a success, else a failure */
    if clean.len() == num {
        Ok(clean)
    } else {
        Err(VaxError::new("idk.jpeg"))
    }
}
fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    let mut input: Vec<String> = Vec::new();

    /* this program requires at least 2 arguments */
    if args.len() < 2 {
        help();
        std::process::exit(1);
    }

    /* put the arguments into an input vector */
    while args.len() > 1 {
        input.push(args.pop().unwrap());
    }

    /* for each file in the command line arguments ... */
    while !input.is_empty() {
        /* read the file name */
        let filename = input.pop().unwrap();
        print!("Cleaning up {}...", filename);

        /* create the vaxuum-ed "*.clean" file name */
        let mut output = String::new();
        output.push_str(&filename);
        output.push_str(".clean");

        /* attempt to open the original file */
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

        /* read in the original file as u8's */
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;

        /* run the de-vax-ify function */
        match devax(buf) {
            Ok(xor) => {
                /* write out the de-vax-ified vector to file */
                println!("file OK!");
                /* attempt to open the new cleaned file */
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

                /* write out the clean version of the file */
                out.write_all(&xor)?;
                out.flush()?;
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}
