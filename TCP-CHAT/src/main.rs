use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{self, Read, Write};
use std::cmp::Ordering;

// Cria o servidor e para o cliente do server ele recebe a mensagem
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

// Cria e conecta o cliente no server e chama enviar mensagem
fn cliente()
{
    let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();

    enviar_mensagem(&mut stream);
}

fn enviar_mensagem(stream: &mut TcpStream)
{
    print!("Digite sua mensagem (Caso deseja encerrar, digite 'exit'): ");
    io::stdout().flush().unwrap();

    let saida = "exit";
    let mut texto = String::new();

    io::stdin().read_line(&mut texto).expect("moio meu vei");

    let mensagem = texto;

    // TEMPORARIO: o melhor seria criar uma fn para lidar com disconnect
    match mensagem.as_str().trim().cmp(saida) 
    {
        Ordering::Less => {}
        Ordering::Greater => {}
        Ordering::Equal => {println!("Saindo... "); let _ = &stream.shutdown(Shutdown::Both);}
    }
    // IMPORTANTE: ta dando panic pq o cliente é desconectado mais a fn continua executando

    // IMPORTANTE: O SERVIDOR APENAS LE BYTES
    stream.write_all(&mensagem.as_bytes()).unwrap();

    println!("Mensagem enviada!");
}

// Aqui ele cria um buffer pra armazenar a msg, dps le do server e armazena ainda como byte
// logo, ele printa ja devolvendo pra string
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