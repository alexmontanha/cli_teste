use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input> [output]", args[0]);
        return;
    } else if args.len() > 3 {
        println!("Too many arguments");
        return;
    } else {
        if args.len() == 3 {
            println!("{:?}_{:?}.out", args[1], args[2]);
        } else {
            println!("{:?}.in", args[1]);
        }
    }
}
