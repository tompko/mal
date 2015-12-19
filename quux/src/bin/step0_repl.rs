extern crate mal;

use mal::readline::{readline, add_history};

fn read(line: String) -> String {
    line
}

fn eval(line: String) -> String {
    line
}

fn print(line: String) -> String {
    line
}

fn rep(line: String) -> String {
    let r = read(line);
    let e = eval(r);
    let p = print(e);
    p
}

fn main() {
    loop {
        let line = readline("user> ").expect("Error reading line");
		add_history(&line);

        let res = rep(line);

        println!("{}", res);
    }
}
