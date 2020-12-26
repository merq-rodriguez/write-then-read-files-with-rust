use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {
    let output = "Hello, world!";

    let mut input = String::new();

    let mut ofile = File::create("hello_world.txt")
        .expect("Unable writting in file");

    //Wirting in file
    ofile.write_all(output.as_bytes())
        .expect("Douhhh error mother fucker writing in file");

    //Open file
    let mut ifile = File::open("hello_world.txt")
        .expect("Unable to open file");

    //Read in file
    ifile.read_to_string(&mut input)
        .expect("Unable to read");
    
    //Compare input with output
    assert_eq!(output, input, "No equals");

}
