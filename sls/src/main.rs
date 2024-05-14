use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::SystemTime;

fn main() {
    let num = fs::read_dir("assets").unwrap().count();
    
    for (i, file) in fs::read_dir("assets").unwrap().enumerate() {
        print!("({i}/{num}) running {:?}...", file.as_ref().unwrap().file_name());
        std::io::stdout().flush().unwrap();
        let start = SystemTime::now();
        Command::new("cadical").stdout(Stdio::null()).arg("-t").arg("3").arg(file.as_ref().unwrap().path()).spawn().unwrap().wait().unwrap();
        let duration = SystemTime::now().duration_since(start).unwrap();

        if duration.as_millis() >= 3000 {
            println!("took too long, deleting");
            std::fs::remove_file(file.as_ref().unwrap().path()).unwrap();
        } else {
            println!("took {}s, keeping", duration.as_secs());
        }
    }
}
