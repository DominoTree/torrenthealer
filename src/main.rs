use mainline::{Dht, Id};
use std::str::FromStr;

struct AppState {
    dht_client: Dht,
    torrent_groups: Vec<TorrentGroup>,
}

const DHT_BOOTSTRAP_NODES: [&str; 7] = [
    "dht.aelitis.com:6881",
    "dht.anacrolix.link:42069",
    "dht.libtorrent.org:25401",
    "dht.transmissionbt.com:6881",
    "router.bitcomet.com:6881",
    "router.bittorrent.com:6881",
    "router.utorrent.com:6881",
];

struct TorrentGroup {
    torrents: Vec<Torrent>,
}

struct Torrent {
    info_hash: String,
    files: Vec<File>,
}

struct File {
    hash: String,
    name: String,
}

fn main() {
    let state = AppState {
        dht_client: Dht::client().unwrap(),
        torrent_groups: vec![],
    };

    loop {
        let dht_client = &state.dht_client;
        let torrent_groups = &state.torrent_groups;

        for torrent_group in torrent_groups {
            for torrent in &torrent_group.torrents {
                for file in &torrent.files {
                    let hash = &file.hash;
                    let name = &file.name;
                    let info_hash = &torrent.info_hash;
                    let peers = dht_client
                        .get_peers(Id::from_str(info_hash).unwrap())
                        .unwrap();
                    for peer in peers {
                        println!("Peer: {} - File: {}", "lol", name);
                    }
                }
            }
        }
    }
}
