use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;



use crate::machine::Inst;

pub fn reader(filename: &str) -> BufReader<File> {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Impossible to open file.");
    BufReader::new(file)
}

pub fn parse_prog(fichier: BufReader<File>) -> Vec<(Option<String>, Inst)> {
    let mut prog: Vec<(Option<String>, Inst)> = Vec::new();
    for l in fichier.lines() {
        let line :String = l.unwrap();
        let mut tokens = line
            .split_whitespace()
            .collect::<Vec<&str>>();
        let mut label = None;

        if !line.starts_with("\t") {
            let mut value: String = tokens[0].to_string();
            value.pop();
            label = Some(value);
            tokens.remove(0);
        }

        let instr: Inst;
        match tokens[0] {
            "CONST" => instr = Inst::Const(tokens[1].parse::<i64>().unwrap()),
            "PRIM" => instr = Inst::Prim(tokens[1].to_string()),
            "BRANCH" => instr = Inst::Branch(tokens[1].to_string()),
            "BRANCHIFNOT" => instr = Inst::BranchIfNot(tokens[1].to_string()),
            "PUSH" => instr = Inst::Push,
            "POP" => instr = Inst::Pop,
            "ACC" => instr = Inst::Acc(tokens[1].parse::<u64>().unwrap()),
            "ENVACC" => instr = Inst::Envacc(tokens[1].parse::<u64>().unwrap()),
            "CLOSURE" => {
                let args = tokens[1].split(',').collect::<Vec<&str>>();
                instr = Inst::Closure(args[0].to_string(), args[1].parse::<i64>().unwrap())
            }
            "APPLY" => instr = Inst::Apply(tokens[1].parse::<i64>().unwrap()),
            "RETURN" => instr = Inst::Return(tokens[1].parse::<i64>().unwrap()),
            "STOP" => instr = Inst::Stop,
            "CLOSUREREC" => {
                let args = tokens[1].split(',').collect::<Vec<&str>>();
                instr = Inst::ClosureRec(args[0].to_string(), args[1].parse::<i64>().unwrap())
            }
            "OFFSETCLOSURE" => instr = Inst::OffSetClosure,
            "GRAB" => instr = Inst::Grab(tokens[1].parse::<i64>().unwrap()),
            "RESTART" => instr = Inst::Restart,
            _ => panic!("instruction non supportée"),
        }
        prog.push((label, instr));
    }

    prog
}
