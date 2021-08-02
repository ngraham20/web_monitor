use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn compare_snapshot() -> Result<()> {
    Ok(())
}

fn save_snapshot(data: &str) -> Result<()> {
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;

    let path = Path::new("snapshot.txt");
    let display = path.display();

    // open file in write only
    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(_) => println!("Successfuly wrote to {}", display),
    }
}

fn load_snapshot() -> Result<()> {
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;

    let path = Path::new("snapshot.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(file) => file,
    };

    match file.read_to_string(buf: &mut String)

}

pub fn take_snapshot(url: &str) -> Result<()> {
    

    

    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let mut prev_snapshot = load_snapshot();

    save_snapshot(&body);

    // println!("Status: {}", res.status());
    // println!("Headers\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    Ok(())
}