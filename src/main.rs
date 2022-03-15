use std::io;
use std::io::BufRead;

fn parse_head<F: Fn(char) -> bool>(cond: F, s: &str) -> Option<(char, &str)> {
    let c = s.chars().nth(0)?;
    if cond(c) {
        Some((c, &s[1..]))
    } else {
        None
    }
}

fn parse_while<F: Fn(char) -> bool>(cond: F, s: &str) -> Option<(&str, &str)> {
    let mut i = 0;
    while i < s.len() {
        if cond(s.chars().nth(i).unwrap()) {
            i += 1;
        } else {
            break;
        }
    }
    if i == 0 {
        None
    } else {
        Some((&s[..i], &s[i..]))
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
        let num_s = parse_while(|c| c.is_digit(10) || c == '-' || c == '.', s)?;
        let num = match num_s.0.parse::<f64>() {
            Ok(n) => n,
            Err(_) => return None,
        };
        Some((Box::new(Literal { num }), num_s.1))
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
        let res: Option<(Box<dyn Node>, &str)> = (|| {
            let s = parse_head(|x| x == '(', s)?.1;
            let (expr, s) = Add::parse(s)?;
            let s = parse_head(|x| x == ')', s)?.1;
            Some((Box::new(Group { expr }) as Box<dyn Node>, s))
        })();
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

struct Add {
    expr1: Box<dyn Node>,
    expr2: Box<dyn Node>,
}

impl Add {
    fn parse(s: &str) -> Option<(Box<dyn Node>, &str)> {
        let res: Option<(Box<dyn Node>, &str)> = (|| {
            let (expr1, s) = Group::parse(s)?;
            let s = parse_head(|x| x == '+', s)?.1;
            let (expr2, s) = Group::parse(s)?;
            Some((Box::new(Add { expr1, expr2 }) as Box<dyn Node>, s))
        })();
        match res {
            None => Group::parse(s),
            some => some,
        }
    }
}

impl Node for Add {
    fn calc(&self) -> f64 {
        self.expr1.calc() + self.expr2.calc()
    }
}

fn parse(s: &str) -> Option<Box<dyn Node>> {
    let expr = Add::parse(s)?;
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
