mod config;
mod handler;
mod protocol;

fn main() {
    let handler = match handler::Handler::new() {
        Ok(handler) => handler,
        Err(error) => {
            pause(error);
            std::process::exit(1);
        }
    };

    match handler.run() {
        Ok(_) => {}
        Err(error) => {
            pause(error);
            std::process::exit(1);
        }
    }
}

fn pause(error: handler::HandlerError) {
    eprint!("{}", error);
    std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();
}
