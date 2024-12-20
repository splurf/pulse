mod base;

use base::*;

use std::{
    io::stdin,
    sync::{Arc, RwLock},
    thread::scope,
    time::Duration,
};

use cpal::traits::{DeviceTrait, StreamTrait};
use opus::{Application, Channels};

pub type Ping = Arc<RwLock<Duration>>;

fn main() -> Result {
    init_logger();
    let cfg = Config::default();

    let (input, output) = {
        let host_devices = Device::get_host_devices()?;

        let (inputs, outputs) = host_devices
            .into_values()
            .next()
            .ok_or(CpalError::Unexpected)?;

        (
            inputs.into_iter().next().ok_or(CpalError::Unexpected)?,
            outputs.into_iter().next().ok_or(CpalError::Unexpected)?,
        )
    };

    // let tcp = TcpClient::new(cfg.addr_remote())?;
    let udp = UdpClient::new(cfg.local_addr(), cfg.remote_addr())?;

    // Buffer to store input samples to send to output
    let (input_sender, input_receiver) = bounded::<Vec<i16>>(1024);
    let (output_sender, output_receiver) = bounded::<Vec<i16>>(1024);

    // Input stream: capture audio and adjust sensitivity
    let input_stream = input
        .build_input_stream(
            input.config(),
            move |data: &[i16], _| {
                if data.iter().any(|&sample| sample.abs() > 32) {
                    input_sender.send(data.to_vec()).unwrap();
                }
            },
            move |err| {
                error!("An error occurred on input stream: {}", err);
            },
            None,
        )
        .unwrap();

    // Output stream: play received audio data from the input stream
    let output_stream = output
        .build_output_stream(
            output.config(),
            move |data: &mut [i16], _| {
                if let Ok(src) = output_receiver.try_recv() {
                    let n = src.len().min(data.len());
                    data[..n].copy_from_slice(&src[..n]);
                    data[n..].fill(0);
                } else {
                    data.fill(0);
                }
            },
            move |err| {
                error!("An error occurred on output stream: {}", err);
            },
            None,
        )
        .unwrap();

    info!("[ IN  ] => {:?}", input);
    info!("[ OUT ] => {:?}", output);

    input_stream.play().unwrap();
    output_stream.play().unwrap();

    scope(move |s| {
        // let ping = Ping::default();

        // let _tcp = handle_tcp(s, tcp, ping);
        let _udp = handle_udp(s, udp, input_receiver, output_sender);
    });

    info!("Press ENTER to exit.");
    stdin().read_line(&mut String::new())?;

    Ok(())
}
