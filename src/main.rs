#[allow(unused_variables)]
#[allow(unused_imports)]
use std::io::{Error, ErrorKind, Write};

const OPERATOR: [char; 4] = ['+', '-', '*', '/'];

const INPUT: &str = "Input:";
const OUTPUT: &str = "Output:";
const MAX: u32 = 10;
const MIN: u32 = 1;

struct Latin<'a> {
    name: &'a str,
    value: u32,
}

fn main() {
    let latins = create_latins();
    loop {
        println!("{}", INPUT);
        let vec = parse_args();
        if vec.len() != 3 {
            println!("Expected 3 argumets received {}", vec.len());
            continue;
        }
        let mut errors = Vec::<Error>::new();
        let mut first_is_latin = false;
        let mut second_is_latin = false;
        let first = parse_number(&vec[0], "first");
        let first = match first {
            Ok(v) => v,
            Err(err) => {
                let first_l = get_latin_value(&latins, &vec[0]);
                let first_l = match first_l {
                    Some(_v) => {
                        first_is_latin = true;
                        _v
                    }
                    None => {
                        errors.push(err);
                        1
                    }
                };

                first_l
            }
        };
        check_min_max(&first, "first", &mut errors);
        let operator = parse_operator(&vec[1]);
        let operator = match operator {
            Ok(v) => v,
            Err(err) => {
                errors.push(err);
                '-'
            }
        };
        let second = parse_number(&vec[2], "second");
        let second = match second {
            Ok(v) => v,
            Err(err) => {
                let second_l = get_latin_value(&latins, &vec[2]);
                let second_l = match second_l {
                    Some(_v) => {
                        second_is_latin = true;
                        _v
                    }
                    None => {
                        errors.push(err);
                        1
                    }
                };
                second_l
            }
        };
        check_min_max(&second, "second", &mut errors);
        if errors.len() != 0 {
            println!("{}", OUTPUT);
            for e in errors {
                println!("{}", e);
            }
            continue;
        }
        let calc_res = calculate(&first, &second, operator);
        if first_is_latin || second_is_latin {
            if calc_res > MAX || calc_res < MIN {
                println!(
                    "Out of range latin result '{}': [{} - {}]",
                    calc_res, MIN, MAX
                );
                continue;
            }
            let latin = get_latin_name(&latins, &calc_res);
            let latin = match latin {
                Some(v) => v,
                None => "I",
            };
            println!("{} {}", OUTPUT, latin);
            continue;
        }
        println!("{} {}", OUTPUT, calc_res);
    }
}

fn get_latin_value(latins: &Vec<Latin>, val: &String) -> Option<u32> {
    let mut res: Option<u32> = None;
    for l in latins {
        if l.name == val {
            res = Some(l.value);
        }
    }
    res
}

fn get_latin_name<'a>(latins: &'a Vec<Latin>, val: &u32) -> Option<&'a str> {
    let mut res: Option<&str> = None;
    for l in latins {
        if l.value == *val {
            res = Some(l.name);
        }
    }
    res
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

fn check_min_max(val: &u32, name: &str, errors: &mut Vec<Error>) {
    if *val > MAX || *val < MIN {
        errors.push(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Error, {} value '{}' out of range: [{} - {}]",
                name, val, MIN, MAX
            ),
        ));
    }
}

fn parse_number(val: &String, name: &str) -> Result<u32, Error> {
    let num = val.parse::<u32>();
    let res: Result<u32, Error> = match num {
        Ok(v) => Ok(v),
        Err(err) => {
            let err_mess = format!("Failed parse {} number: '{}', {:?}", name, val, err);
            Err(Error::new(ErrorKind::InvalidInput, err_mess))
        }
    };
    res
}

fn calculate(first: &u32, second: &u32, operator: char) -> u32 {
    match operator {
        '+' => first + second,
        '-' => first - second,
        '*' => first * second,
        '/' => first % second,
        _ => 0,
    }
}

fn parse_operator(val: &String) -> Result<char, Error> {
    let error = true;
    let mut res: Result<char, Error> = Err(Error::new(
        ErrorKind::InvalidInput,
        format!(
            "Operator is missing, expected one of {:?}, received '{}'",
            &OPERATOR, val
        ),
    ));
    for o in &OPERATOR {
        if *val == o.to_string() {
            res = Ok(*o);
        }
    }
    res
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

fn create_latins<'a>() -> Vec<Latin<'a>> {
    vec![
        Latin {
            name: "I",
            value: 1,
        },
        Latin {
            name: "II",
            value: 2,
        },
        Latin {
            name: "III",
            value: 3,
        },
        Latin {
            name: "IV",
            value: 4,
        },
        Latin {
            name: "V",
            value: 5,
        },
        Latin {
            name: "VI",
            value: 6,
        },
        Latin {
            name: "VII",
            value: 7,
        },
        Latin {
            name: "VIII",
            value: 8,
        },
        Latin {
            name: "IX",
            value: 9,
        },
        Latin {
            name: "X",
            value: 10,
        },
    ]
}
