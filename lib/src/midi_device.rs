use std::error::Error;
use std::io::stdin;
use midly::{live::LiveEvent, MidiMessage, num::u4};
use midir::{MidiInput, MidiOutput};
use midir::os::unix::{VirtualInput, VirtualOutput};

#[derive(Debug, Clone, Copy)]
pub struct MidiData {
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

// Implements MidiAction by passing `channel` as is and acting on MidiMessage
pub trait MidiActionPassChannel {
  fn midi_action_on_msg<O>(&mut self, data : MidiMessage, outport : O ) where 
    O: FnMut(MidiMessage);
}

impl<A: MidiActionPassChannel> MidiAction for A {
  fn midi_action<O>(&mut self, data : MidiData, mut outport : O ) where 
    O: FnMut(MidiData) {
    self.midi_action_on_msg(data.message, |msg| outport(MidiData::new(data.channel, msg)));
  }
}

// Says what to do with an incoming MIDI message
pub trait MidiAction {
  fn midi_action<O>(&mut self, data : MidiData, outport : O ) where 
    O: FnMut(MidiData);
}

#[cfg(not(target_arch = "wasm32"))] // conn_out is not `Send` in Web MIDI, which means it cannot be passed to connect
pub fn create_virtual_midi_device<A>(device_name : &str, mut action : A) -> Result<(), Box<dyn Error>>  // no value in defining concrete error type
    where 
    A: MidiAction + Send + 'static
    {

    let midi_in = MidiInput::new(device_name)?;
    // midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new(device_name)?;
    let mut port_out = midi_out.create_virtual( "out")?;
    
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let port_in = midi_in.create_virtual(
        "in",
        move |_stamp, message, _| {
            let event = LiveEvent::parse(message).unwrap();
            let mut send_out_event = |event: LiveEvent<'_> | {
              let mut buf = Vec::new();
              event.write(&mut buf).unwrap();
              port_out
                  .send(&buf[..])
                  .unwrap_or_else(|e| println!("Error when forwarding message: {:?}", e));
          };
            let send_out_msg = | note : MidiData | send_out_event(LiveEvent::from(note));
            match event {
                LiveEvent::Midi { channel, message } => {
                    action.midi_action( MidiData::new(channel, message), send_out_msg );
                    // send_out_msg(out_msg);
                }
                x => {
                    send_out_event(x);
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
