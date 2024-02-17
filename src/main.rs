extern crate portmidi;
use std::thread::sleep;
use std::time::Duration;
use portmidi::{MidiMessage, PortMidiDeviceId};

static MIDI_CHANNEL: u8 = 0;    // == 1
static MIDI_CONTROLLER_ID: PortMidiDeviceId = 8;

fn main() {
    let mut context = portmidi::PortMidi::new().unwrap();
    let mut pm = context
        .device(MIDI_CONTROLLER_ID)
        .and_then(|dev| context.output_port(dev, 1024))
        .unwrap();
    let port_number = MIDI_CHANNEL;
    let channel = 0; // MIDI channel 1
    let controller = 64; // Sustain CC#
    let value = 127; // Pedal full to the floor

    let kevs_cc = MidiMessage {
        status: 0x90 + port_number,
        data1: controller,
        data2: value,
        data3: 0,
    };
    pm.write_message(kevs_cc).unwrap();
    println!("KJSL: Sent MIDI CC: channel {}, controller {}, value {}", channel, controller, value);

    sleep(Duration::from_millis(2000));

    let kevs_cc2 = MidiMessage {
        status: 0x90 + port_number,
        data1: controller,
        data2: 0,
        data3: 0,
    };

    let value = 0; // Release sustain pedal
    pm.write_message(kevs_cc2).unwrap();
    println!("KJSL: Sent MIDI CC: channel {}, controller {}, value {}", channel, controller, value);
}