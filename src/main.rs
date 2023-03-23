use clap::{Arg, Command};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::new("App")
        .arg(
            Arg::new("IP")
                .long("ip")
                .short('i')
                .help("IP of the challenge"),
        )
        .arg(
            Arg::new("Port")
                .long("port")
                .short('p')
                .help("Port of the challenge"),
        )
        .get_matches();
    let host = app.get_one::<String>("IP").unwrap();
    let port = app.get_one::<String>("Port").unwrap();
    let target = format!("{}:{}", host, port);
    let connection = TcpStream::connect(&target)?;
    let mut reader = BufReader::new(&connection);
    let mut writer = BufWriter::new(&connection);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_until(b'>', &mut buffer)?;
    reader.read_until(b'>', &mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer));
    buffer.clear();
    writer.write("1\n".as_bytes())?;
    writer.flush()?;
    reader.read_until(b'>', &mut buffer)?;
    reader.read_until(b'>', &mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer));
    for _ in 1..100 {
        play_game(&mut writer, &mut reader)?;
    }
    Ok(())
}

fn play_game(
    writer: &mut BufWriter<&TcpStream>,
    reader: &mut BufReader<&TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut time_seed: i64 = 0;
        let raw_time_seed = &mut time_seed as *mut i64;
        let time = libc::time(raw_time_seed);
        libc::srand(time as u32);
        let random_number = libc::rand();
        let weapon = vec!["paper", "rock", "scissors"];
        let choice = random_number % 3;
        writer.write(weapon[choice as usize].as_bytes())?;
        writer.flush()?;
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'>', &mut buffer)?;
        reader.read_until(b'>', &mut buffer)?;
        println!("{}", String::from_utf8_lossy(&buffer));
    }
    Ok(())
}
