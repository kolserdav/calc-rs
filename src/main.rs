#[allow(unused_variables)]
use std::io::Write;

#[derive(Debug)]
struct Latins<'a> {
    name: &'a str,
    value: u8,
}

fn main() {
    // let latins = create_latins();
    loop {
        let vec = parse_args();
        for i in &vec {
            let first = i.parse::<u8>();
            if let Err(err) = first {
                println!("Error parse first number {:?}", err);
            }
            
        }
    }
    /*
    for l in &latins {
        println!("{:?}", l.name);
    }
    */
}

fn split_line<'a>(line: &str) -> core::str::Split<&'a str> {
    line.split(" ")
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

fn parse_line() -> String {
    let stdin = std::io::stdin().lines();
    let mut line: String = String::from("");
    for l in stdin {
        line = l.unwrap();
        break;
    }
    return line;
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
