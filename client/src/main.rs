mod base;

use base::*;

use std::{io::stdin, sync::Arc, thread::scope, time::Duration};

use cpal::traits::{DeviceTrait, StreamTrait};

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

    println!("INPUT => {:?}", input);
    println!("OUTPUT => {:?}", output);

    // let tcp = TcpClient::new(cfg.addr())?;
    let udp = UdpClient::new(cfg.addr_local(), cfg.addr_remote())?;

    // Buffer to store input samples to send to output
    let (input_sender, input_receiver) = unbounded::<Vec<f32>>();
    let (output_sender, output_receiver) = unbounded::<Vec<f32>>();

    // Input stream: capture audio and adjust sensitivity
    let input_stream = input
        .build_input_stream(
            &input.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if data.iter().any(|&sample| sample > 0.01) {
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
            &output.config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                if let Ok(adjusted_data) = output_receiver.recv() {
                    // Fill output buffer with adjusted data
                    for (out_sample, &in_sample) in data.iter_mut().zip(adjusted_data.iter()) {
                        *out_sample = in_sample;
                    }
                }
            },
            move |err| {
                error!("An error occurred on output stream: {}", err);
            },
            None,
        )
        .unwrap();

    debug!("{:?}", input);
    debug!("{:?}", output);

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
