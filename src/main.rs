mod server;
mod request;

use server::Server;

fn main() {
    let server = Server::create("127.0.0.1:7878");
    server.watch();
}
