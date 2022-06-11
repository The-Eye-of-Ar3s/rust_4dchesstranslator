use std::{env, process, path::Path, fs};
use regex::Regex;

fn main() {
    let filename = arg_collector();
    let filecontents = file_reader(filename);
    let cleancode = cleaner(filecontents);
    let optcode = optimizer(cleancode);
    let output = translator(optcode.0, optcode.1);
    fs::write("out.cpp", output).expect("Unable to write!");
}

fn arg_collector() -> String{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("NO FILE SPECIFIED!");
        process::exit(1);
    }
    let arg = args.remove(1);
    return arg;
}

fn file_reader(filename: String) -> String{
    if Path::new(&filename).is_file() {
        let raw = fs::read_to_string(filename).expect("Failed to read File!");
        return raw;
    } else {
        eprintln!("Specified File was not found!");
        process::exit(1);
    }
}

fn cleaner(raw: String) -> String{
    let expr = Regex::new(r"(?i)[^<|>|\+|\-|,|\.|\[|\]|^|v|*|o|@|?]").unwrap();
    let result = expr.replace_all(&raw, "").to_string();
    return [result, "!".to_string()].join("").to_string();
}

fn optimizer(code: String) -> (Vec<String>, Vec<usize>){
    let mut keyvec: Vec<String> = Vec::new();
    let mut countvec: Vec<usize> = Vec::new();
    let cmdvec: Vec<char> = code.chars().collect();
    let mut cmd;
    let mut prev: String = "".to_string();
    let mut tmp;
    let mut cp: &str;
    for cmdchar in cmdvec {
        cmd = cmdchar.to_string();
        if prev != cmd || ".[]".to_string().contains(&cmd) {
            cp = &cmd;
            prev = cp.to_string();
            keyvec.push(cp.to_string());
            countvec.push(1);
        } else {
            tmp = countvec.pop().unwrap();
            tmp += 1;
            countvec.push(tmp as usize);
        }
    }
    return (keyvec, countvec);
}

fn translator(keyvec: Vec<String>, countvec: Vec<usize>) -> String {
    let mut outcode = vec!["#include <stdio.h>".to_owned(),"int main() {".to_owned(),"int array[8][8][8][8] = {0};".to_owned(),"int idx[4] = {0, 0, 0, 0};".to_owned()];
    for index in 0..keyvec.len() {
        match keyvec[index].as_str() {
            "." => {
                outcode.push("putchar(array[idx[0]][idx[1]][idx[2]][idx[3]]);".to_owned());
            }
            "," => {
                outcode.push("array[idx[0]][idx[1]][idx[2]][idx[3]] = getchar();".to_owned());
            }
            "[" => {
                outcode.push("while (array[idx[0]][idx[1]][idx[2]][idx[3]]) {".to_owned());
            }
            "]" => {
                outcode.push("}".to_owned());
            }
            ">" => {
                outcode.push(format!("idx[0] += {};", countvec[index]));
            }
            "<" => {
                outcode.push(format!("idx[0] -= {};", countvec[index]));
            }
            "^" => {
                outcode.push(format!("idx[1] += {};", countvec[index]));
            }
            "v" => {
                outcode.push(format!("idx[1] -= {};", countvec[index]));
            }
            "*" => {
                outcode.push(format!("idx[2] += {};", countvec[index]));
            }
            "o" => {
                outcode.push(format!("idx[2] -= {};", countvec[index]));
            }
            "@" => {
                outcode.push(format!("idx[3] += {};", countvec[index]));
            }
            "?" => {
                outcode.push(format!("idx[3] -= {};", countvec[index]));
            }
            "+" => {
                outcode.push(format!("array[idx[0]][idx[1]][idx[2]][idx[3]] += {};", countvec[index]));
            }
            "-" => {
                outcode.push(format!("array[idx[0]][idx[1]][idx[2]][idx[3]] -= {};", countvec[index]));
            }
            _ => {}
        }
    }
    outcode.push("}".to_owned());
    println!("{:?}:{:?}", keyvec, countvec);
    return outcode.join("\n");
}