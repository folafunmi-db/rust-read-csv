use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records() {
        let a = result?;
        println!("{:?}", a);
    }
    // reader.records().into_iter().map(|x| {
    //     let a = x?;
    //     println!("{:?}", a);
    // });

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./file.csv") {
        eprintln!("{}", e);
    }
}
