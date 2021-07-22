use handler::Handler;

mod config;
mod handler;
mod protocol;

fn main() {
    let handler = match Handler::new() {
        Ok(handler) => handler,
        Err(error) => {
            eprintln!("{}", error);
            std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();
            std::process::exit(1);
        }
    };

    match handler.run() {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();
            std::process::exit(1);
        }
    }
}
