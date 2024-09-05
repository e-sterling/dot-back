use std::env;
use std::path::PathBuf;
use homedir::my_home;

#[derive(Debug)]
struct AppConfig {
    backup_dest: PathBuf,
    backup_type: String,
}

fn main() {
    
    let mut app_config = AppConfig {
        backup_dest: my_home()
            .expect("Unable to locate home directory")
            .expect("Error reading home directory"),
        backup_type: String::from("file"),
    };

    let mut args = env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-b" | "--backup" => {
                if let Some(cmd_dst) = args.peek() {
                    app_config.backup_dest = PathBuf::from(cmd_dst);
                }
            }
            _ => (),
        }
    }

    dbg!(app_config);
}
