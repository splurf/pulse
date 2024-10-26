use opus::{Application, Channels, Decoder, Encoder};

use crate::*;

use std::thread::{Scope, ScopedJoinHandle};

fn _compress_audio(pcm_samples: Vec<i16>, sample_rate: u32) -> Result<Vec<u8>> {
    // Set up the Opus encoder for 1-channel (mono) audio.
    let channels = Channels::Mono;
    let mut encoder = Encoder::new(sample_rate, channels, Application::Audio).unwrap();

    // // Convert `f32` samples (-1.0 to 1.0) to `i16` samples (-32768 to 32767)
    // let pcm_samples: Vec<i16> = data
    //     .iter()
    //     .map(|&sample| (sample * i16::MAX as f32) as i16)
    //     .collect();

    // Prepare a buffer to store the compressed data
    let max_packet_size = 65536; // Opus packets are typically small
    let mut compressed_data = vec![0; max_packet_size];

    // Compress the PCM data using Opus
    let compressed_size = encoder.encode(&pcm_samples, &mut compressed_data).unwrap();

    // Truncate to the actual compressed size
    compressed_data.truncate(compressed_size);

    Ok(compressed_data)
}

fn _decode_audio(compressed_data: &[u8], sample_rate: u32, frame_size: usize) -> Result<Vec<i16>> {
    // Set up the Opus decoder for 1-channel (mono) audio.
    let channels = Channels::Mono;
    let mut decoder = Decoder::new(sample_rate, channels).unwrap();

    // Prepare a buffer to hold the decoded PCM samples
    let mut pcm_samples = vec![0; frame_size * channels as usize]; // Adjust size according to frame size and channels

    // Decode the compressed audio
    let decoded_samples = decoder
        .decode(compressed_data, &mut pcm_samples, false)
        .unwrap();

    // Truncate to the actual number of decoded samples
    pcm_samples.truncate(decoded_samples * channels as usize);

    Ok(pcm_samples)
}

fn handle_input<'a>(
    s: &'a Scope<'a, '_>,
    mut udp: UdpClient,
    input_receiver: Receiver<Vec<f32>>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || -> Result<()> {
        udp.socket()
            .set_write_timeout(Some(Duration::from_millis(1)))?;

        // repeatedly send user input to server
        loop {
            let input = input_receiver.recv()?;

            // let data = compress_audio(input, 48_000)?;

            // all in one
            udp.send(&Packet::Data(input))?;
        }
    })
}

pub fn handle_udp<'a, 'b: 'a>(
    s: &'a Scope<'a, '_>,
    udp: UdpClient,
    input_receiver: Receiver<Vec<f32>>,
    output_sender: Sender<Vec<f32>>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || {
        let _ = handle_input(s, udp.try_clone()?, input_receiver);

        let mut buf = init_buf();

        loop {
            // wait for server to send player update
            let packet: Packet = udp.recv(&mut buf, PacketKind::Data)?;

            // debug!("{:?}", packet);

            match packet {
                Packet::Data(output) => {
                    // let data = decode_audio(&output, 48_000, 960)?;
                    output_sender.send(output)?
                }
                _ => (),
            }
        }
    })
}
