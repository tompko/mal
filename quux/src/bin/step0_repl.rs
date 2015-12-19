use std::io;
use std::io::Write;

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
        print!("user>a ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .ok()
            .expect("Error reading line");

        let res = rep(line);

        print!("{}", res);
        io::stdout().flush().unwrap();
    }
}
