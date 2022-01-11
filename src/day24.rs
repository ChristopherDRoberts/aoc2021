use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut registers = HashMap::<&str, Exp>::new();
    registers.insert("x", Exp::Constant(0));
    registers.insert("y", Exp::Constant(0));
    registers.insert("z", Exp::Constant(0));
    registers.insert("w", Exp::Constant(0));

    let instructions = read_input(input);
    let mut variables = vec![
        "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "d11", "d12", "d13", "d14",
    ];
    for ins in instructions {
        match ins[0] {
            "inp" => {
                registers
                    .insert(ins[1], Exp::Variable(variables.pop().unwrap().to_owned()))
                    .unwrap();
            }
            "add" => {
                if let Ok(v) = ins[2].parse::<isize>() {
                    registers
                        .insert(
                            ins[1],
                            Exp::add(registers[ins[1]].clone(), Exp::Constant(v)),
                        )
                        .unwrap();
                } else {
                    registers
                        .insert(
                            ins[1],
                            Exp::add(registers[ins[1]].clone(), registers[ins[2]].clone()),
                        )
                        .unwrap();
                }
            }
            "mul" => {
                if let Ok(v) = ins[2].parse::<isize>() {
                    registers
                        .insert(
                            ins[1],
                            Exp::mul(registers[ins[1]].clone(), Exp::Constant(v)),
                        )
                        .unwrap();
                } else {
                    registers
                        .insert(
                            ins[1],
                            Exp::mul(registers[ins[1]].clone(), registers[ins[2]].clone()),
                        )
                        .unwrap();
                }
            }
            "div" => {
                if let Ok(v) = ins[2].parse::<isize>() {
                    registers
                        .insert(
                            ins[1],
                            Exp::div(registers[ins[1]].clone(), Exp::Constant(v)),
                        )
                        .unwrap();
                } else {
                    registers
                        .insert(
                            ins[1],
                            Exp::div(registers[ins[1]].clone(), registers[ins[2]].clone()),
                        )
                        .unwrap();
                }
            }
            "mod" => {
                if let Ok(v) = ins[2].parse::<isize>() {
                    registers
                        .insert(
                            ins[1],
                            Exp::modulo(registers[ins[1]].clone(), Exp::Constant(v)),
                        )
                        .unwrap();
                } else {
                    registers
                        .insert(
                            ins[1],
                            Exp::modulo(registers[ins[1]].clone(), registers[ins[2]].clone()),
                        )
                        .unwrap();
                }
            }
            "eql" => {
                if let Ok(v) = ins[2].parse::<isize>() {
                    registers
                        .insert(
                            ins[1],
                            Exp::eql(registers[ins[1]].clone(), Exp::Constant(v)),
                        )
                        .unwrap();
                } else {
                    registers
                        .insert(
                            ins[1],
                            Exp::eql(registers[ins[1]].clone(), registers[ins[2]].clone()),
                        )
                        .unwrap();
                }
            }
            _ => panic!(),
        }
        let s = registers["z"].clone().string();
        println!("{}", s);
    }
    0
}

pub fn part2(input: &str) -> usize {
    0
}

fn read_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|l| l.trim().split(" ").collect())
        .collect()
}

#[derive(Clone)]
enum Exp {
    Constant(isize),
    Variable(String),
    Eql(Box<Exp>, Box<Exp>),
    Add(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Div(Box<Exp>, Box<Exp>),
    Mod(Box<Exp>, Box<Exp>),
}

impl Exp {
    fn add(lhs: Exp, rhs: Exp) -> Exp {
        match (&lhs, &rhs) {
            (Exp::Constant(0), _) => rhs,
            (_, Exp::Constant(0)) => lhs,
            (Exp::Constant(l), Exp::Constant(r)) => Exp::Constant(l + r),
            (_, _) => Exp::Add(Box::new(lhs.clone()), Box::new(rhs.clone())),
        }
    }

    fn mul(lhs: Exp, rhs: Exp) -> Exp {
        match (&lhs, &rhs) {
            (Exp::Constant(0), _) => Exp::Constant(0),
            (_, Exp::Constant(0)) => Exp::Constant(0),
            (Exp::Constant(1), _) => rhs,
            (_, Exp::Constant(1)) => lhs,
            (Exp::Constant(l), Exp::Constant(r)) => Exp::Constant(l * r),
            (_, _) => Exp::Mul(Box::new(lhs.clone()), Box::new(rhs.clone())),
        }
    }

    fn div(lhs: Exp, rhs: Exp) -> Exp {
        match (&lhs, &rhs) {
            (_, Exp::Constant(0)) => panic!(),
            (Exp::Constant(0), _) => Exp::Constant(0),
            (_, Exp::Constant(1)) => lhs,
            (Exp::Constant(l), Exp::Constant(r)) => Exp::Constant(l / r),
            (_, _) => Exp::Div(Box::new(lhs.clone()), Box::new(rhs.clone())),
        }
    }

    fn modulo(lhs: Exp, rhs: Exp) -> Exp {
        match (&lhs, &rhs) {
            (Exp::Constant(0), _) => Exp::Constant(0),
            (_, Exp::Constant(1)) => Exp::Constant(0),
            (Exp::Constant(l), Exp::Constant(r)) => Exp::Constant(l % r),
            (_, _) => Exp::Mod(Box::new(lhs), Box::new(rhs)),
        }
    }

    fn eql(lhs: Exp, rhs: Exp) -> Exp {
        match (&lhs, &rhs) {
            (Exp::Constant(l), Exp::Constant(r)) => Exp::Constant(if l == r { 1 } else { 0 }),
            (Exp::Variable(l), Exp::Variable(r)) => Exp::Constant(if l == r { 1 } else { 0 }),
            (_, _) => Exp::Eql(Box::new(lhs), Box::new(rhs)),
        }
    }

    fn string(&self) -> String {
        match self {
            Exp::Constant(c) => c.to_string(),
            Exp::Variable(v) => v.clone(),
            Exp::Eql(lhs, rhs) => format!("({} == {})", lhs.string(), rhs.string()),
            Exp::Add(lhs, rhs) => format!("({} + {})", lhs.string(), rhs.string()),
            Exp::Mul(lhs, rhs) => format!("({} * {})", lhs.string(), rhs.string()),
            Exp::Div(lhs, rhs) => format!("({} / {})", lhs.string(), rhs.string()),
            Exp::Mod(lhs, rhs) => format!("({} % {})", lhs.string(), rhs.string()),
        }
    }
}

#[cfg(test)]
mod tests_day24 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 12
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 7
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 13
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 8
        mul y x
        add z y
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 13
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 10
        mul y x
        add z y
        inp w";
        let result = part1(input);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part2() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 112);
    }
}
