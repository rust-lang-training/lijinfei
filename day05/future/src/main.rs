use async_std::{net, io::{WriteExt, ReadExt}};
#[async_std::main]
async fn main() {
    let result = cheapo_request("localhost", 8080, "/").await;
    match result {
        Ok(x) => {
            println!("content is {}", x)   
        },
        Err(_x) => {
            println!("unknow");
        }
    }
}
async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    Ok(response)
}