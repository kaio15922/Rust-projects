use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

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

    print!("Digite sua mensagem... ");
    io::stdout().flush().unwrap();

    enviar_mensagem(&mut stream);
}

fn enviar_mensagem(stream: &mut TcpStream)
{
    let mut texto = String::new();

    io::stdin().read_line(&mut texto).expect("moio meu vei");

    let mensagem = texto;

    stream.write_all(&mensagem.as_bytes()).unwrap();

    println!("Mensagem enviada!");
}

fn receber_mensagem(stream: &mut TcpStream)
{
    let mut buffer = [0; 1024];

    let bytes = stream.read(&mut buffer).unwrap();

    print!(
        "Recebido: {}",
        String::from_utf8_lossy(&buffer[..bytes])
    );
    io::stdout().flush().unwrap();
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