use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;

use tiles::{TileArchive, TileId};

pub struct MapServer {
    address: SocketAddr,
}

impl MapServer {
    const ANY_PORT: &str = "127.0.0.1:0";
    const ASSETS: &str = "assets";
    const PAGE: &str = include_str!("../../assets/map_check.html");
    const ARCHIVE: &str = "assets/basho.tiles";
    const JS_REPLY: &str = "text/javascript";
    const PAGE_REPLY: &str = "text/html";
    const OTHER_REPLY: &str = "application/octet-stream";

    pub fn started() -> MapServer {
        let listener = TcpListener::bind(Self::ANY_PORT).expect("a local port opens");
        let address = listener.local_addr().expect("the port has an address");
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let archive =
            Arc::new(TileArchive::open(&root.join(Self::ARCHIVE)).expect("the tiles open"));
        std::thread::spawn(move || {
            listener
                .incoming()
                .map_while(Result::ok)
                .for_each(|stream| {
                    let archive = Arc::clone(&archive);
                    std::thread::spawn(move || Self::answered(stream, &archive));
                });
        });
        MapServer { address }
    }

    pub fn page(&self) -> String {
        format!("http://{}/check", self.address)
    }

    fn answered(mut stream: TcpStream, archive: &TileArchive) -> Option<()> {
        let mut line = String::new();
        BufReader::new(&stream).read_line(&mut line).ok()?;
        let path = line.split(' ').nth(1)?;
        let body = Self::body(path, archive);
        let kind = match path.rsplit('.').next() {
            Some("js") => Self::JS_REPLY,
            Some("/check") => Self::PAGE_REPLY,
            Some(_) | None => Self::OTHER_REPLY,
        };
        let head = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(head.as_bytes()).ok()?;
        stream.write_all(&body).ok()
    }

    fn body(path: &str, archive: &TileArchive) -> Vec<u8> {
        let file = path.strip_prefix('/').unwrap_or(path);
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        match file.split_once('/') {
            Some((Self::ASSETS, _)) => fs::read(root.join(file)).unwrap_or_default(),
            Some(("tiles", name)) => name
                .parse::<TileId>()
                .ok()
                .and_then(|id| archive.tile(id).ok().flatten())
                .unwrap_or_default(),
            _ => Self::PAGE.as_bytes().to_vec(),
        }
    }
}
