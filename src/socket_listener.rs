use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::net::UnixStream;

fn read_socket(socket_addr: String) -> std::io::Result<()> {
    let stream = match UnixStream::connect(socket_addr) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect : {e:?}");
            return Err(e);
        }
    };
    let mut reader = BufReader::new(stream);
    loop {
        let mut buff: Vec<u8> = vec![];
        reader.read_until(b'\n', &mut buff).unwrap();
        let data = String::from_utf8_lossy(&buff);
        println!("{}", data);
        let parts: Vec<&str> = data.trim().split(">>").collect();
        if parts[0] == "monitoradded" {
            // Check monitor type
            // apply configuration
        }
    }
}
