use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn gen_random_number() -> u16 {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

fn create_file(file_name: String) -> File {
    return File::create(file_name).expect("Unable writting in file");
}

fn write_file(mut file:  File, text: &String) -> File {
    file.write_all(text.as_bytes())
        .expect("Douhhh error mother fucker writing in file");
    return file;
}

fn read_file(file_name: String) -> File {
    let mut input = String::new();
    let mut file = File::open(file_name).expect("Unable to open file");

    file.read_to_string(&mut input).expect("Unable to read");
    return file;
}

fn getSample() {
    let output = "Hello, world!";

    let mut input = String::new();

    let mut ofile = File::create("hello_world.txt").expect("Unable writting in file");

    //Wirting in file
    ofile
        .write_all(output.as_bytes())
        .expect("Douhhh error mother fucker writing in file");

    //Open file
    let mut ifile = File::open("hello_world.txt").expect("Unable to open file");

    //Read in file
    ifile.read_to_string(&mut input).expect("Unable to read");

    //Compare input with output
    assert_eq!(output, input, "No equals");
}

fn main() {
  let mut i = 1;

  while i < 10 {

    let random = gen_random_number();
    let file_name = String::from(i.to_string() + ".txt");
    let name = file_name.clone();

    println!("{}) {}", i, random);
    let  file = create_file(name);

    write_file(file, &random.to_string());
    i = i + 1;
  }
}
