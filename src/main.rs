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
}

struct Torrent {
    info_hash: String,
    peers: Vec<SocketAddr>,
    trackers: Vec<SocketAddr>,
}

impl Torrent {
    fn get_peers(&mut self) {}
}

fn main() {
    let info_hash = read_to_string("magnets.txt").unwrap();

    let state = AppState {
        dht_client: Dht::client().unwrap(),
    };

    let dht_client = &state.dht_client;

    let mut res = dht_client
        .get_peers(Id::from_str(&info_hash.trim()).unwrap())
        .unwrap();

    loop {
        let lol = res.next();
        println!("{:?}", lol);

        if lol.is_none() {
            break;
        }
    }
}
