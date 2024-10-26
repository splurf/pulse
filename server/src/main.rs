mod base;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    thread::scope,
};

use base::*;

pub type TcpClients = Arc<RwLock<HashMap<u8, TcpClient>>>;
pub type UdpClients = Arc<RwLock<HashSet<SockAddr>>>;

fn main() -> Result {
    init_logger();
    let cfg = Config::default();

    // // init TCP server
    // let tcp = TcpListener::bind(cfg.addr())?;
    // info!("TCP @ {}", tcp.local_addr()?);

    // init UDP server
    let udp = UdpServer::new(cfg.addr_local())?;
    info!("UDP @ {:?}", udp.socket().local_addr()?);

    udp.socket().set_recv_buffer_size(65536)?;

    // // map of player streams
    // let clients_tcp: TcpClients = Default::default();

    // map of player data
    let clients_udp: UdpClients = Default::default();

    // // share client UDP sockets between main thread and 'alive' thread
    // // TODO => 'heartbeat' for bi-direction
    // let (sender_addr, receiver_addr) = unbounded::<SocketAddr>();

    // // share client TCP packets
    // let (sender_packet, receiver_packet) = unbounded::<Packet>();

    // // monotonic user identity
    // let id = Arc::new(AtomicU8::new(1));

    // init networking
    scope(move |s| -> Result<()> {
        // // handle TCP packets
        // init_tcp(
        //     s,
        //     tcp,
        //     clients_tcp.clone(),
        //     clients_udp.clone(),
        //     sender_packet,
        //     receiver_addr,
        //     receiver_packet,
        // );

        // handle UDP packets
        handle_udp(s, udp.try_clone()?, udp, clients_udp);

        Ok(())
    })?;

    Ok(())
}
