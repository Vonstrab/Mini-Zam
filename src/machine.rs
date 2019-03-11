use crate::mlvalue::Mlvalue;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Inst {
    Const(i64),
    Prim(String),
    Branch(String),
    BranchIfNot(String),
    Push,
    Pop,
    Acc(u64),
    Envacc(u64),
    Closure(String, i64),
    Apply(i64),
    Return(i64),
    Stop,
    //fermeture recurcive
    ClosureRec(String, i64),
    OffSetClosure,
    //fonction n aire
    Grab(i64),
    Restart,
}

pub struct ZAM {
    pub prog: Vec<(Option<String>, Inst)>,
    pub stack: Vec<Mlvalue>,
    pub env: Vec<Mlvalue>,
    pub pc: usize,
    pub accu: Mlvalue,
    pub extra_args: usize,
    pub labelsMap: HashMap<String, usize>,
    pub options: Vec<String>,
}

impl ZAM {
    pub fn new(prog: &Vec<(Option<String>, Inst)>) -> ZAM {
        let mut labels: HashMap<String, usize> = HashMap::new();

        for (i, value) in prog.into_iter().enumerate() {
            let val = value.0.clone();
            if !val.is_none() {
                let label: String = val.unwrap();
                labels.insert(label, i);
            }
        }

        ZAM {
            prog: prog.clone(),
            stack: Vec::new(),
            env: Vec::new(),
            pc: 0,
            accu: Mlvalue::Entier(0),
            extra_args: 0,
            labelsMap: labels,
            options: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut run = true;
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        while run {
            let inst = self.prog[self.pc as usize].1.clone();

            if self.options.contains(&"Debug".to_string()) {
                println!("La Stack {:?}", self.stack);
                println!("L'env {:?}", self.env);
                println!("L'accu {:?}", self.accu);
                println!("extras args {:?}", self.extra_args);
                println!("\nInstruction {} : {:?}", self.pc, inst);
            }

            if self.options.contains(&"Step".to_string()) {
                // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
                write!(stdout, "\nPress enter to continue...").unwrap();
                stdout.flush().unwrap();

                // Read a single byte and discard
                let _ = stdin.read(&mut [0u8]).unwrap();
            }

            self.execute(&inst);

            if inst == Inst::Stop {
                run = false;
            }
        }
    }

    pub fn set_option(&mut self, option: &str) {
        if !self.options.contains(&option.to_string()) {
            self.options.push(option.to_string());
        }
    }

    pub fn execute(&mut self, instruction: &Inst) {
        match instruction {
            Inst::Const(n) => {
                self.accu = Mlvalue::Entier(*n);
                self.pc += 1;
            }
            Inst::Prim(op) => {
                match op.as_str() {
                    "+" => {
                        let a0: i64 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() + a0;
                        self.accu = Mlvalue::Entier(result);
                    }
                    "/" => {
                        let a0: i64 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() / a0;
                        self.accu = Mlvalue::Entier(result);
                    }
                    "-" => {
                        let a0: i64 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() - a0;
                        self.accu = Mlvalue::Entier(result);
                    }
                    "*" => {
                        let a0: i64 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() * a0;
                        self.accu = Mlvalue::Entier(result);
                    }
                    "not" => {
                        let result = !self.accu.as_bool();
                        self.accu = Mlvalue::to_entier(result);
                    }
                    "<" => {
                        let a0 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() < a0;
                        self.accu = Mlvalue::to_entier(result);
                    }
                    ">" => {
                        let a0 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() > a0;
                        self.accu = Mlvalue::to_entier(result);
                    }
                    "=" => {
                        let a0 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() == a0;
                        self.accu = Mlvalue::to_entier(result);
                    }
                    "<=" => {
                        let a0 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() <= a0;
                        self.accu = Mlvalue::to_entier(result);
                    }
                    ">=" => {
                        let a0 = self.stack.pop().unwrap().as_int();
                        let result = self.accu.as_int() >= a0;
                        self.accu = Mlvalue::to_entier(result);
                    }
                    "or" => {
                        let a0 = self.stack.pop().unwrap().as_bool();
                        let result = self.accu.as_bool() || a0;
                        self.accu = Mlvalue::to_entier(result);
                    }
                    "and" => {
                        let a0 = self.stack.pop().unwrap().as_bool();
                        let result = self.accu.as_bool() && a0;
                        self.accu = Mlvalue::to_entier(result);
                    }
                    "print" => {
                        let val = self.accu.as_int();
                        print!("{}", (val as u8) as char)
                    }
                    _ => {
                        panic!("Operation non supportee");
                    }
                }
                self.pc += 1;
            }
            Inst::Branch(l) => self.pc = *self.labelsMap.get(&l.clone()).unwrap(),
            Inst::BranchIfNot(l) => {
                if self.accu == Mlvalue::Entier(0) {
                    self.pc = *self.labelsMap.get(&l.clone()).unwrap();
                } else {
                    self.pc += 1;
                }
            }
            Inst::Push => {
                self.stack.push(self.accu.clone());
                self.pc += 1;
            }
            Inst::Pop => {
                self.stack.pop();
                self.pc += 1;
            }
            Inst::Acc(i) => {
                let indice: usize = self.stack.len() - (*i as usize) - 1;
                self.accu = self.stack[indice].clone();
                self.pc += 1;
            }
            Inst::Envacc(i) => {
                self.accu = self.env[*i as usize].clone();
                self.pc += 1;
            }
            Inst::Closure(l, n) => {
                if *n > 0 {
                    let acc = self.accu.clone();
                    self.stack.push(acc);
                }
                let posl = *self.labelsMap.get(&l.clone()).unwrap();
                let mut nenv = Vec::new();
                for _ in 0..*n {
                    nenv.push(self.stack.pop().unwrap());
                }
                self.accu = Mlvalue::Fermeture(posl, nenv);
                self.pc += 1;
            }
            Inst::Apply(n) => {
                if *n < 0 {
                    panic!("Nombre D'arguments négatifs");
                }
                let mut tmp = Vec::new();
                for _ in 0..*n {
                    tmp.push(self.stack.pop().unwrap());
                }

                self.stack.push(Mlvalue::Entier(self.extra_args as i64));
                self.stack.push(Mlvalue::Entier((self.pc + 1) as i64));
                self.stack.push(Mlvalue::Environement(self.env.clone()));

                for i in 0..*n {
                    let val = tmp[i as usize].clone();
                    self.stack.push(val);
                }

                match &self.accu {
                    Mlvalue::Fermeture(npc, nenv) => {
                        self.pc = *npc;
                        self.env = nenv.clone();
                    }
                    _ => panic!("Pas de Fermeture dans l'Accu"),
                }

                self.extra_args = (*n as usize) - 1;
            }
            Inst::Return(n) => {
                if *n < 0 {
                    panic!("Nombre D'Arguments Négatifs");
                }

                for _ in 0..*n {
                    self.stack.pop();
                }

                if self.extra_args == 0 {
                    self.env = self.stack.pop().unwrap().as_env();
                    self.pc = self.stack.pop().unwrap().as_int() as usize;
                    self.extra_args = self.stack.pop().unwrap().as_int() as usize;
                } else {
                    self.extra_args -= 1;
                    match &self.accu {
                        Mlvalue::Fermeture(npc, nenv) => {
                            self.pc = *npc;
                            self.env = nenv.clone();
                        }
                        _ => panic!("Not a Fermeture"),
                    }
                }
            }
            Inst::Stop => {
                println!("Valeur de fin de Code {:?}", self.accu);
            }
            //Fonctions Recursives
            Inst::ClosureRec(l, n) => {
                if *n > 0 {
                    let acc = self.accu.clone();
                    self.stack.push(acc);
                }
                let posl = *self.labelsMap.get(&l.clone()).unwrap();
                let mut nenv = Vec::new();
                nenv.push(Mlvalue::Entier(posl as i64));
                for _ in 0..*n {
                    nenv.push(self.stack.pop().unwrap());
                }
                self.accu = Mlvalue::Fermeture(posl, nenv);
                self.stack.push(self.accu.clone());
                self.pc += 1;
            }
            Inst::OffSetClosure => {
                self.accu = Mlvalue::Fermeture(self.env[0].as_int() as usize, self.env.clone());
                self.pc += 1;
            }
            //Fonctionsn aires
            Inst::Grab(n) => {
                if self.extra_args >= *n as usize {
                    self.extra_args -= *n as usize;
                    self.pc -= 1;
                } else {
                    for _ in 0..self.extra_args  {
                        self.env.push(self.stack.pop().unwrap());
                    }
                    self.accu = Mlvalue::Fermeture(self.pc - 1, self.env.clone());

                    self.extra_args = self.stack.pop().unwrap().as_int() as usize;
                    self.pc = self.stack.pop().unwrap().as_int() as usize;
                    self.env = self.stack.pop().unwrap().as_env();
                }
            }
            Inst::Restart => {
                let n = self.env.len();
                if n > 0 {
                    for i in 1..n {
                        self.stack.push(self.env[i].clone());
                    }
                    self.env = vec![self.env[0].clone()];
                    self.extra_args += (n - 1);
                }
                self.pc += 1;
            }
        }
    }
}
