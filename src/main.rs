#[allow(unused_variables)]
#[allow(unused_imports)]
use std::io::{Error, ErrorKind, Write};

#[derive(Debug)]
struct Latins<'a> {
    name: &'a str,
    value: u8,
}

fn main() {
    // let latins = create_latins();
    loop {
        let vec = parse_args();
        if vec.len() != 3 {
            println!("Expected 3 argumets received {}", vec.len());
            continue;
        }
        let mut errors = Vec::<Error>::new();
        parse_number(&vec[0], "first", &mut errors);
        parse_number(&vec[2], "second", &mut errors);
        if errors.len() != 0 {
            println!("Some values wrong:");
            for e in errors {
                println!("{}", e);
            }
            continue;
        }
    }
    /*
    for l in &latins {
        println!("{:?}", l.name);
    }
    */
}

fn parse_args<'a>() -> Vec<String> {
    let line_p = parse_line();
    let line = line_p.split(" ");
    let mut vec = Vec::new();
    for l in line {
        vec.push(l.to_string());
    }
    vec
}

fn parse_number(val: &String, name: &str, errors: &mut Vec<Error>) {
    let first = val.parse::<u8>();
    if let Err(err) = first {
        let err_mess = format!("Failed parse {} number: '{}', {:?}", name, val, err);
        errors.push(Error::new(ErrorKind::InvalidInput, err_mess));
    }
}

fn parse_line() -> String {
    let stdin = std::io::stdin().lines();
    let mut line: String = String::from("");
    for l in stdin {
        line = l.unwrap();
        break;
    }
    return line;
}

#[allow(dead_code)]
fn split_line<'a>(line: &str) -> core::str::Split<&'a str> {
    line.split(" ")
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn create_latins<'b>() -> Vec<Latins<'b>> {
    let mut latins = Vec::new();
    latins.push(Latins {
        name: "I",
        value: 1,
    });
    latins.push(Latins {
        name: "II",
        value: 2,
    });
    latins.push(Latins {
        name: "III",
        value: 3,
    });
    latins.push(Latins {
        name: "IV",
        value: 4,
    });
    latins.push(Latins {
        name: "V",
        value: 5,
    });
    latins.push(Latins {
        name: "VI",
        value: 6,
    });
    latins.push(Latins {
        name: "VII",
        value: 7,
    });
    latins.push(Latins {
        name: "VIII",
        value: 8,
    });
    latins.push(Latins {
        name: "IX",
        value: 9,
    });
    latins.push(Latins {
        name: "X",
        value: 10,
    });
    latins
}
