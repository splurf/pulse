use crate::*;

use std::{
    thread::{Scope, ScopedJoinHandle},
    time::Instant,
};

// bi-direction pinging
pub fn handle_tcp<'a, 'b: 'a>(
    s: &'a Scope<'a, '_>,
    tcp: TcpClient,
    ping: Ping,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || {
        let mut buf = [0; PACKET_SIZE];

        loop {
            // init timer
            let time = Instant::now();

            // ping to server
            tcp.send(&Packet::Ping)?;

            // wait for response
            let _: Packet = tcp.recv(&mut buf, PacketKind::empty())?;

            // update ping nonetheless
            *ping.write() = time.elapsed()
        }
    })
}
