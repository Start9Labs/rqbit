use super::pex_peers_for;

#[test]
fn public_pex_recipients_never_receive_local_peer_addresses() {
    let peers: Vec<std::net::SocketAddr> = [
        "10.0.0.1:1",
        "172.16.1.2:1",
        "192.168.1.2:1",
        "127.0.0.1:1",
        "169.254.1.2:1",
        "[::1]:1",
        "[fd00::1]:1",
        "[fe80::1]:1",
        "[::ffff:192.168.1.2]:1",
        "203.0.113.2:1",
    ]
    .into_iter()
    .map(|peer| peer.parse().unwrap())
    .collect();
    for recipient in ["203.0.113.1", "2001:db8::1"] {
        assert_eq!(
            pex_peers_for(recipient.parse().unwrap(), peers.iter().copied()).collect::<Vec<_>>(),
            vec!["203.0.113.2:1".parse().unwrap()]
        );
    }
}

#[test]
fn local_pex_recipients_keep_local_discovery() {
    let peers = [
        "192.168.1.2:1".parse().unwrap(),
        "[fd00::2]:1".parse().unwrap(),
    ];
    for recipient in ["192.168.1.1", "fe80::1"] {
        assert_eq!(
            pex_peers_for(recipient.parse().unwrap(), peers.into_iter()).collect::<Vec<_>>(),
            peers
        );
    }
}

#[test]
fn local_candidates_do_not_starve_the_public_pex_batch() {
    let private = std::iter::repeat_n("10.0.0.1:1".parse().unwrap(), 60);
    let public = (1..=60).map(|port| std::net::SocketAddr::from(([203, 0, 113, 1], port)));
    let sent =
        pex_peers_for("203.0.113.2".parse().unwrap(), private.chain(public)).collect::<Vec<_>>();
    assert_eq!(sent.len(), 50);
    assert_eq!(sent.first().unwrap().port(), 1);
    assert_eq!(sent.last().unwrap().port(), 50);
}
