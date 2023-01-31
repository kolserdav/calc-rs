#[derive(Debug)]
struct Latins<'a> {
    name: &'a str,
    value: u8,
}

fn main() {
    let latins = create_latins();
    loop {
        parse_args();
    }
    /*
    for l in &latins {
        println!("{:?}", l.name);
    }
    */
}

fn split_line(line: &str) {
    print_type_of(&line.split(" "));
}

fn parse_args<'a>() -> Vec<&'a str> {
    let line = parse_line();
    let line = line.split(" ");
    let vec = line.collect();
    return vec;
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
