use std::io;
use std::io::BufRead;

fn parse_char(c: char, s: &str) -> Option<&str> {
    if s.chars().nth(0)? == c {
        Some(&s[1..])
    } else {
        None
    }
}

trait Node {
    fn calc(&self) -> f64;
}

struct Literal {
    num: f64,
}

impl Literal {
    fn parse(s: &str) -> Option<(Box<dyn Node>, &str)> {
        None
    }
}

impl Node for Literal {
    fn calc(&self) -> f64 {
        self.num
    }
}

struct Group {
    expr: Box<dyn Node>,
}

impl Group {
    fn parse(s: &str) -> Option<(Box<dyn Node>, &str)> {
        let res: Option<(Box<dyn Node>, &str)> = {
            let s = parse_char('(', s)?;
            let (expr, s) = Group::parse(s)?;
            let s = parse_char(')', s)?;
            Some((Box::new(Group { expr }), s))
        };
        match res {
            None => Literal::parse(s),
            some => some,
        }
    }
}

impl Node for Group {
    fn calc(&self) -> f64 {
        self.expr.calc()
    }
}

fn parse(s: &str) -> Option<Box<dyn Node>> {
    let expr = Group::parse(s)?;
    if expr.1.len() > 0 {
        None
    } else {
        Some(expr.0)
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(s) => {
                if s == "quit" {
                    return;
                }
                let tree = match parse(s.as_str()) {
                    Some(x) => x,
                    None => {
                        eprintln!("Couldn't parse input expression.");
                        continue;
                    }
                };
                let result = tree.calc();
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("Error reading in from stdin: {:?}", e);
                std::process::exit(-1);
            }
        }
    }
}
