use opus::{Bitrate, Decoder, Encoder};

use crate::*;

use std::thread::{Scope, ScopedJoinHandle};

// Constants
pub const SAMPLE_RATE: u32 = 48_000; // 48 kHz
pub const FRAME_SIZE: usize = 960; // 20 ms at 48 kHz
pub const BITRATE: Bitrate = Bitrate::Bits(64_000); // 64 kbps (adjust as needed)

/// Encodes audio using the Opus codec
fn encode_audio(encoder: &mut Encoder, input: &[i16]) -> Result<Vec<u8>, Error> {
    // Prepare the output buffer (Opus is compressed, so buffer size can be smaller)
    let mut output = vec![0u8; input.len() * 2]; // Overestimate buffer size

    // Encode the PCM data
    let size = encoder.encode(input, &mut output).unwrap();

    output.truncate(size);

    // Return the encoded data (only the valid encoded portion)
    Ok(output)
}

/// Decodes audio using the Opus codec
fn decode_audio(decoder: &mut Decoder, input: &[u8]) -> Result<Vec<i16>, Error> {
    // Prepare the output buffer (PCM data in 16-bit samples)
    let mut output = vec![0i16; FRAME_SIZE];

    // Decode the Opus data
    let size = decoder.decode(input, &mut output, false).unwrap();

    output.truncate(size);

    // Return the encoded data (only the valid encoded portion)
    Ok(output)
}

fn handle_input<'a>(
    s: &'a Scope<'a, '_>,
    mut udp: UdpClient,
    input_receiver: Receiver<Vec<i16>>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || -> Result<()> {
        udp.socket()
            .set_write_timeout(Some(Duration::from_millis(1)))?;

        // Initialize the Opus encoder for audio applications with voice focus
        let mut encoder = Encoder::new(SAMPLE_RATE, Channels::Mono, Application::Audio).unwrap();
        encoder.set_bitrate(BITRATE).unwrap();

        // repeatedly send user input to server
        loop {
            let input = input_receiver.recv()?;

            let data = encode_audio(&mut encoder, input.as_slice()).unwrap();

            // all in one
            udp.send(&Packet::Data(data))?;
        }
    })
}

pub fn handle_udp<'a, 'b: 'a>(
    s: &'a Scope<'a, '_>,
    udp: UdpClient,
    input_receiver: Receiver<Vec<i16>>,
    output_sender: Sender<Vec<i16>>,
) -> ScopedJoinHandle<'a, Result> {
    s.spawn(move || {
        let _ = handle_input(s, udp.try_clone()?, input_receiver);

        // Initialize the Opus decoder
        let mut decoder = Decoder::new(SAMPLE_RATE, Channels::Mono).unwrap();

        let mut buf = init_buf();

        loop {
            // wait for server to send player update
            let packet: Packet = udp.recv(&mut buf, PacketKind::Data)?;

            // debug!("{:?}", packet);

            if let Packet::Data(output) = packet {
                let data = decode_audio(&mut decoder, output.as_slice())?;
                output_sender.send(data)?
            }
        }
    })
}
