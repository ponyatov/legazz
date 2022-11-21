#![allow(unused_variables)]
#![allow(dead_code)]

// use memmap::Mmap;
// use std::{env, fs::File};

mod lexer;
mod parser;

fn arg(argc: usize, argv: &str) {
    println!("argv[{}] = <{}>", argc, argv);
}

fn main() {
    // arg
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        // arg
        arg(argc, argv);
        // // file
        // let src = std::fs::read_to_string(argv).expect(argv);
        // dbg!(src);
        // mmap
        {
            let file = std::fs::File::open(argv).expect(argv);
            let mmap = unsafe { memmap::Mmap::map(&file).expect(&format!("{:?}", file)) };
            let lexer = lexer::input(mmap, argv);
            dbg!(std::str::from_utf8(lexer.token()).unwrap());
            // drop(file); // close
        }
    }
}
