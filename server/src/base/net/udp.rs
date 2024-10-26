use crate::*;

use std::{
    sync::Arc,
    thread::{Scope, ScopedJoinHandle},
    time::Duration,
};

use spin_sleep::SpinSleeper;

fn handle_dist<'a>(
    s: &'a Scope<'a, '_>,
    receiver: Receiver<Packet>,
    updated: Arc<Mutex<Vec<Vec<f32>>>>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || loop {
        let packet = receiver.recv()?;
        if let Packet::Data(data) = packet {
            updated.lock().push(data);
        }
    })
}

fn handle_write<'a>(
    s: &'a Scope<'a, '_>,
    udp: UdpServer,
    clients_udp: UdpClients,
    sender: Sender<Packet>,
    receiver_packet: Receiver<Packet>,
    voice: Arc<Mutex<Vec<Vec<f32>>>>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || {
        s.spawn(move || {
            let delay = Duration::from_secs_f32((1000.0 / 1024.0) / 1000.0);
            let spinner = SpinSleeper::default();

            loop {
                spinner.sleep(delay);

                // distribute updates to each player
                for data in voice.lock().drain(..) {
                    for addr in clients_udp.read().iter() {
                        if let Err(e) = udp.send_to(&Packet::Data(data.clone()), addr.clone()) {
                            error!("{:?}", e)
                        }
                    }
                }
            }
        });

        loop {
            let packet = receiver_packet.recv()?;

            // send data to writer thread
            if sender.try_send(packet).is_err() {
                // skip if msg failed to send
                continue;
            }
        }
    })
}

pub fn init_write<'a>(
    s: &'a Scope<'a, '_>,
    udp: UdpServer,
    clients_udp: UdpClients,
    receiver_packet: Receiver<Packet>,
) -> ScopedJoinHandle<'a, ()> {
    s.spawn(move || {
        let (sender, receiver) = bounded(32);
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
    sender_packet: Sender<Packet>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || {
        let mut buf = init_buf();

        loop {
            match udp.recv_from(&mut buf, PacketKind::Data) {
                Ok((packet, addr)) => {
                    clients_udp.write().insert(addr);
                    sender_packet.try_send(packet).unwrap_or_default();
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
        let (sender_packet, receiver_packet) = bounded(32);

        // handle outgoing
        init_write(s, udp_a, clients_udp.clone(), receiver_packet);

        // handle incoming UDP packets
        handle_incoming(s, udp_b, clients_udp, sender_packet);
    })
}
