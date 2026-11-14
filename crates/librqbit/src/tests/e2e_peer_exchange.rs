use std::time::Duration;

use buffers::ByteBuf;
use peer_binary_protocol::extended::handshake::ExtendedHandshake;
use tokio::io::AsyncReadExt;

use super::fake_peer::{FakePeer, SeederOpts, peer_id, run, start_seeder};

#[tokio::test(flavor = "multi_thread")]
async fn disabled_peer_exchange_is_not_advertised_on_the_wire() -> anyhow::Result<()> {
    run(async {
        let seed = start_seeder(SeederOpts {
            disable_peer_exchange: true,
            ..Default::default()
        })
        .await?;
        let mut peer =
            FakePeer::connect(seed.addr(), seed.info_hash, &peer_id(b"-FKPEX1-")).await?;
        peer.extended_handshake(b"d1:md6:ut_pexi1eee").await?;
        let frame = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let length = peer.stream.read_u32().await?;
                let mut message = vec![0; length as usize];
                peer.stream.read_exact(&mut message).await?;
                if message.starts_with(&[20, 0]) {
                    return Ok::<_, std::io::Error>(message);
                }
            }
        })
        .await??;
        let handshake: ExtendedHandshake<ByteBuf<'_>> = bencode::from_bytes(&frame[2..]).unwrap();
        assert_eq!(handshake.m.ut_pex, None);
        assert!(handshake.m.ut_metadata.is_some());
        Ok(())
    })
    .await
}
