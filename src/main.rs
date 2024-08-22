use std::fs::read_to_string;
use std::net::SocketAddr;
use std::str::FromStr;

use mainline::{Dht, Id};

const DHT_BOOTSTRAP_NODES: [&str; 7] = [
    "dht.aelitis.com:6881",
    "dht.anacrolix.link:42069",
    "dht.libtorrent.org:25401",
    "dht.transmissionbt.com:6881",
    "router.bitcomet.com:6881",
    "router.bittorrent.com:6881",
    "router.utorrent.com:6881",
];

struct AppState {
    dht_client: Dht,
    // using a vec here will throw everything onto the heap by default, which saves us the trouble
    // of allocating torrent members there and referencing them
    torrents: Vec<Torrent>,
}

struct Torrent {
    dht_client: Dht,
    info_hash: String,
    peers: Vec<SocketAddr>,
    trackers: Vec<SocketAddr>,
    content: Option<Vec<u8>>,
}

impl Torrent {
    fn get_peers(&mut self) {
        let mut res = self
            .dht_client
            .get_peers(Id::from_str(&self.info_hash).unwrap())
            .unwrap();

        loop {
            let next = res.next();
            if next.is_none() {
                break;
            }
            println!("{:?}", next);
        }
    }

    fn get_content(&mut self) {}
}

fn main() {
    let info_hash = read_to_string("magnets.txt").unwrap();

    let state = AppState {
        dht_client: Dht::client().unwrap(),
        torrents: Vec::new(),
    };

    let mut torrent = Torrent {
        peers: Vec::new(),
        trackers: Vec::new(),
        dht_client: state.dht_client.clone(),
        info_hash: info_hash.trim().to_string(),
        content: None,
    };

    torrent.get_peers();
}
