use clap::{Parser, Subcommand};
use rust_test_crate_add as add;
use rust_test_crate_divide as divide;
use rust_test_crate_multiply as multiply;


/// Simple program to add, divide, or multiply two numbers
#[derive(Parser)]
#[command(name = "rust-test-crate")]
#[command(about = "A simple CLI example using clap to add, divide, or multiply two numbers", long_about = None)]
struct Args {
    /// Operation
    #[arg(short, long)]
    op: String,

    /// First value
    #[arg(short, long)]
    a: isize,

    /// Second value
    #[arg(short, long)]
    b: isize,
}

fn main() {
    let args = Args::parse();
    if args.op == "add" {
        println!("Sum: {}", add::add(args.a, args.b));
    } else if args.op == "divide"{
        println!("Quotient: {}", divide::divide(args.a, args.b));
    } else if args.op == "multiply" {
        println!("Product: {}", multiply::multiply(args.a, args.b));
    } else {
        panic!("An unexpected operation.");
    }
}