extern crate portmidi;

fn main() {
    let mut pm = portmidi::MidiOutput::new().unwrap();
    let port_number = 0;
    let channel = 0; // MIDI channel 1
    let controller = 64; // Sustain CC#
    let value = 127; // Pedal full to the floor
    pm.send_cc(port_number, channel, controller, value).unwrap();
    println!("KJSL: Sent MIDI CC: channel {}, controller {}, value {}", channel, controller, value);

    sleep(2000);

    let value = 0; // Release sustain pedal
    pm.send_cc(port_number, channel, controller, value).unwrap();
    println!("KJSL: Sent MIDI CC: channel {}, controller {}, value {}", channel, controller, value);
}