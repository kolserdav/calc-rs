#[derive(Debug)]
struct Latins<'a> {
    name: &'a str,
    value: u8,
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

fn main() {
    let latins = create_latins();
    for l in &latins {
        println!("{:?}", l.name);
    }
}
