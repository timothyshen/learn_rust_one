use csv;
use std::error::Error;
use std::env;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records(){
        let record = result?;

        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if let Err(e) = read_from_file(&args[1]){
        eprintln!("{}", e);
    }
}

