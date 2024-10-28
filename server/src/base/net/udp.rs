use crate::*;

use std::{
    sync::Arc,
    thread::{Scope, ScopedJoinHandle},
    time::Duration,
};

use spin_sleep::SpinSleeper;

type Updates = Arc<Mutex<Vec<(Vec<u8>, SockAddr)>>>;

fn handle_dist<'a>(
    s: &'a Scope<'a, '_>,
    receiver: Receiver<(Packet, SockAddr)>,
    updated: Updates,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || loop {
        let (packet, addr) = receiver.recv()?;
        if let Packet::Data(data) = packet {
            updated.lock().push((data, addr));
        }
    })
}

fn handle_write<'a>(
    s: &'a Scope<'a, '_>,
    udp: UdpServer,
    clients_udp: UdpClients,
    sender: Sender<(Packet, SockAddr)>,
    receiver_packet: Receiver<(Packet, SockAddr)>,
    voice: Updates,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || {
        s.spawn(move || {
            let delay = Duration::from_secs_f32((1000.0 / 512.0) / 1000.0);
            let spinner = SpinSleeper::default();

            udp.socket()
                .set_write_timeout(Some(Duration::from_millis(1)))
                .unwrap();

            let mut disconnected = Vec::new();

            loop {
                spinner.sleep(delay);

                // distribute updates to each player
                for (data, from) in voice.lock().drain(..) {
                    for addr in clients_udp.read().iter() {
                        if addr == &from {
                            continue;
                        }

                        if let Err(e) = udp.send_to(&Packet::Data(data.clone()), addr) {
                            error!("{}", e);
                            disconnected.push(addr.clone());
                        }
                    }
                }

                if !disconnected.is_empty() {
                    let mut clients = clients_udp.write();

                    disconnected.drain(..).for_each(|addr| {
                        clients.remove(&addr);
                    });
                }
            }
        });

        loop {
            let msg = receiver_packet.recv()?;

            // send data to writer thread
            _ = sender.try_send(msg);
        }
    })
}

pub fn init_write<'a>(
    s: &'a Scope<'a, '_>,
    udp: UdpServer,
    clients_udp: UdpClients,
    receiver_packet: Receiver<(Packet, SockAddr)>,
) -> ScopedJoinHandle<'a, ()> {
    s.spawn(move || {
        let (sender, receiver) = unbounded();
        let updated: Arc<Mutex<Vec<_>>> = Default::default();

        handle_dist(s, receiver, updated.clone());
        handle_write(
            s,
            udp,
            clients_udp,
            sender,
            receiver_packet,
            updated.clone(),
        );
    })
}

fn handle_incoming<'a>(
    s: &'a Scope<'a, '_>,
    udp: UdpServer,
    clients_udp: UdpClients,
    sender_packet: Sender<(Packet, SockAddr)>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || {
        let mut buf = init_buf();

        loop {
            match udp.recv_from(&mut buf, PacketKind::Data) {
                Ok((packet, addr)) => {
                    clients_udp.write().insert(addr.clone());
                    _ = sender_packet.try_send((packet, addr));
                }
                Err(e) => {
                    error!("{:?}", e);
                    break Ok(());
                }
            }
        }
    })
}

pub fn handle_udp<'a>(
    s: &'a Scope<'a, '_>,
    udp_a: UdpServer,
    udp_b: UdpServer,
    clients_udp: UdpClients,
) -> ScopedJoinHandle<'a, ()> {
    s.spawn(move || {
        // real-time game data channel
        let (sender_packet, receiver_packet) = unbounded();

        // handle outgoing
        init_write(s, udp_a, clients_udp.clone(), receiver_packet);

        // handle incoming UDP packets
        handle_incoming(s, udp_b, clients_udp, sender_packet);
    })
}
