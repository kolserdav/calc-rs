#[derive(Debug)]
struct Latins<'a> {
    name: &'a str,
    value: u8,
}

fn main() {
    let latins = create_latins();
    loop {
        let stdin = std::io::stdin().lines();
        let mut line: Option<String> = None;
        for l in stdin {
            line = Some(l.unwrap());
            break;
        }
        if let Some(v) = line {
            let line_i = v.split(" ");
            let vec: Vec<&str> = line_i.collect();
            for i in &vec {
                println!("{:?}", i);
            }
        } else {
            println!("Line not set");
        }
    }
    /*
    for l in &latins {
        println!("{:?}", l.name);
    }
    */
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
