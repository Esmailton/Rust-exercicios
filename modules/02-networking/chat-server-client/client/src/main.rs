use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;


const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Falha ao se conectar ao Stream");
    client.set_nonblocking(true).expect("Falha ao iniciar o non-blocking");

    let (tx,rx) = mpsc::channel::<String>();
    
    thread::spawn(move || loop{
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff){
            Ok(_) =>{
                let msg = buff.into_iter().take_while(|&x| x !=0).collect::<Vec<_>>();
                print!("Mesagem recebida {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Conectado com o servidor pronta");
                break;
            }
        }
        match rx.try_recv(){
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Falha ao escrever para o socket");
                println!("Mensagem enviada {}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Escrevendo mensagem");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Falha ao ler STDIN!");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err(){break}
    }
    println!("Tchau!");
}
