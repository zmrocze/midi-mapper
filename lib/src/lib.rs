use std::error::Error;
use std::io::{stdin, stdout, Write};
use midly::{live::LiveEvent, MidiMessage, num::u4};
use midir::{Ignore, MidiIO, MidiInput, MidiOutput};
use midir::os::unix::{VirtualInput, VirtualOutput};

#[derive(Debug, Clone, Copy)]
struct MidiData {
    channel : u4,
    message : MidiMessage,
}

impl MidiData {
    fn new(channel : u4, message : MidiMessage) -> Self {
        Self { channel, message }
    }
}

impl<'a> From<MidiData> for LiveEvent<'a> {
    fn from(data : MidiData) -> Self {
        LiveEvent::Midi { channel: data.channel, message: data.message }
    }
}

#[cfg(not(target_arch = "wasm32"))] // conn_out is not `Send` in Web MIDI, which means it cannot be passed to connect
pub fn create_virtual_midi_device<T, F>(device_name : &str, mut map : F) -> Result<(), Box<dyn Error>> 
    where 
    // F: FnMut(u64, &[u8], &mut T) -> (u4, MidiMessage) + Send + 'static,
    F: FnMut(MidiData) -> MidiData + Send + 'static,
    T: Send {

    let midi_in = MidiInput::new(device_name)?;
    // midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new(device_name)?;
    let mut port_out = midi_out.create_virtual( "out")?;
    
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let port_in = midi_in.create_virtual(
        "in",
        move |_stamp, message, _| {
            let event = LiveEvent::parse(message).unwrap();
            let mut send_out_msg = |event : LiveEvent<'_>| {
                let mut buf = Vec::new();
                event.write(&mut buf).unwrap();
                port_out
                    .send(&buf[..])
                    .unwrap_or_else(|e| println!("Error when forwarding message: {:?}", e));
            };
            match event {
                LiveEvent::Midi { channel, message } => {
                    let out_msg: LiveEvent<'_> = map( MidiData::new(channel, message)).into();
                    send_out_msg(out_msg);
                }
                x => {
                    send_out_msg(x);
                }
            }
        },
        (),
    )?;
    println!("Created virtual MIDI device {} with in and out ports", device_name);
    let mut input = String::new();
    stdin().read_line(&mut input)?; // wait for next enter key press
    
    port_in.close();
    println!("Closed connections");
    Ok(())
}
