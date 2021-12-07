use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::ddc::InputSource;
use bytes::BytesMut;

pub async fn start<T: Into<String>>(ip: T, port: u16) -> crate::Result<()>{
    let ip = ip.into();
    let listener = TcpListener::bind(format!("{}:{}",ip,port)).await?;

    println!("Listener server bound at {}:{}",ip,port);

    loop {
        println!("Waiting for client");

        let (mut stream, addr) = listener.accept().await?;
        println!("Client at {} accepted",addr);

        tokio::spawn(async move {
            match responder(&mut stream).await {
                Ok(_) => {},
                Err(e) => eprintln!("{:?}",e)
            }
            match stream.shutdown().await{
                Ok(_) => {},
                Err(e) => eprintln!("{:?}",e)
            }
        });
    }
}

pub async fn responder(stream: &mut TcpStream) -> crate::Result<()>{
    loop {
        println!("Waiting for response from client");
        let mut buf = BytesMut::with_capacity(32);
        while !buf.contains(&b'\n'){
            stream.read_buf(&mut buf).await?;
        }

        let sbuf = String::from_utf8_lossy(&buf).trim().to_string();

        println!("Command received: {}",&sbuf);

        handle_command(sbuf, stream).await?;

        println!("Response sent");
    }

}

pub async fn handle_command(sbuf: String, stream: &mut TcpStream) -> crate::Result<()>{
    if sbuf == "read" {
        let input = match crate::ddc::get_input_source(){
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}",e);
                stream.write(b"err\n").await?;
                return Ok(());
            }
        };
        println!("Responding to read with {}",input.value());
        stream.write(input.value().to_string().as_bytes()).await?;
        stream.write(b"\n").await?;
    }
    else if sbuf.starts_with("write"){
        write_command(sbuf, stream).await?;
    }
    else {
        stream.write("err\n".as_bytes()).await?;
        eprintln!("Invalid command received");
    }

    Ok(())
}

pub async fn write_command(text: String, stream: &mut TcpStream) -> crate::Result<()>{
    let remainder = text.trim_start_matches("write 0x");

    let num = match u8::from_str_radix(remainder,16){
        Ok(n) => n,
        _ => {
            eprintln!("Write err invalid input code {}",remainder);
            writeln(stream,String::from("err").as_bytes()).await?;
            return Ok(());
        }
    };

    let input = match InputSource::from_value(num){
        Some(e) => e,
        _ => {
            eprintln!("Invalid input source code");
            writeln(stream,String::from("err").as_bytes()).await?;
            return Ok(());
        }
    };

    match crate::ddc::set_input_source(input){
        Ok(_) => writeln(stream,String::from("ok").as_bytes()).await?,
        Err(_) => writeln(stream,String::from("err").as_bytes()).await?,
    };

    println!("Response to write finished");

    Ok(())
}

async fn writeln(stream: &mut TcpStream, msg: &[u8]) -> crate::Result<()>{
    let mut v = msg.to_vec();
    v.push(b'\n');
    stream.write_all(&v).await?;
    Ok(())
}