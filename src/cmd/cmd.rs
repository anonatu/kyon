
use std::env;
use



pub enum  Cmd{
   CmdLineCounter(String),
}



fn get_params() -> Result<Cmd,Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let mut app = String::new();
    app=args[1].clone();
    match app {
        "line_counter" => {
            let mut path = String::new();
            path=args[2].clone();
            (app, path)
        },
        _ => {
            println!("{} is not a valid command", app);
            process::exit(1);
        }
    }

}
