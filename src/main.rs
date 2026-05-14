use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};

fn servidor()
{
    let server = TcpListener::bind("127.0.0.1:5000").unwrap();

    println!("Servidor ligou!");

    for stream in server.incoming()
    {
        let mut stream = stream.unwrap();

        receber_mensagem(&mut stream);
    }
}

fn cliente()
{
    let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();

    enviar_mensagem(&mut stream);
}

fn enviar_mensagem(stream: &mut TcpStream)
{
    stream.write_all(b"ola tudo bem?").unwrap();

    println!("Mensagem enviada!");
}

fn receber_mensagem(stream: &mut TcpStream)
{
    let mut buffer = [0; 1024];

    let bytes = stream.read(&mut buffer).unwrap();

    println!(
        "Recebido: {}",
        String::from_utf8_lossy(&buffer[..bytes])
    )
}
fn main()
{
    let modo = std::env::args().nth(1);

    match modo.as_deref()
    {
        Some("server") => servidor(),
        Some("client") => cliente(),
        _ => println!("Use server ou client"),
    }
}