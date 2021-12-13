use midir::{
  MidiInput, MidiInputConnection, MidiInputPort, MidiOutput, MidiOutputConnection, MidiOutputPort,
};

use rand::Rng;

use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;

pub struct Launchpad {
  color_map: HashMap<u8, u8>,

  daw_conn: MidiOutputConnection,
  midi_conn: MidiOutputConnection,

  midi_in: Option<MidiInputConnection<()>>,
}

fn empty_pad_state() -> HashMap<u8, u8> {
  let mut hm = HashMap::new();

  for i in 11..100 {
    hm.insert(i, 0);
  }

  hm
}

pub fn midi_input(text: &'static str) -> MidiInput {
  MidiInput::new(text).unwrap()
}

pub fn midi_output(text: &'static str) -> MidiOutput {
  MidiOutput::new(text).unwrap()
}

pub fn print(s: &str) {
  print!("{}", s);
  stdout().flush().unwrap();
}

pub fn input(prompt: &str) -> String {
  print(prompt);
  read!("{}\n")
}

fn input_usize(prompt: &str) -> usize {
  print(prompt);
  read!("{}\n")
}

pub fn position_to_note(x: u8, y: u8) -> u8 {
  81 - (10 * y) + x
}

pub fn get_midi_in_ports(conn: &MidiInput) -> (MidiInputPort, MidiInputPort) {
  let in_ports = conn.ports();

  for port in in_ports.clone() {
    println!("{:?}", conn.port_name(&port).unwrap());
  }

  let get_port = move |name: &'static str| {
    in_ports
      .clone()
      .into_iter()
      .find(|p| conn.port_name(p).unwrap() == name)
      .unwrap()
  };

  let daw_conn = get_port("Launchpad X LPX DAW Out");
  let midi_out = get_port("Launchpad X LPX MIDI Out");

  (daw_conn, midi_out)
}

pub fn get_midi_out_ports(out: &MidiOutput) -> (MidiOutputPort, MidiOutputPort) {
  let out_ports = out.ports();

  let get_port = move |name: &'static str| {
    out_ports
      .clone()
      .into_iter()
      .find(|p| out.port_name(p).unwrap() == name)
      .unwrap()
  };

  let daw_conn = get_port("Launchpad X LPX DAW In");
  let midi_out = get_port("Launchpad X LPX MIDI In");

  (daw_conn, midi_out)
}

pub fn initialize_output() -> (MidiOutputConnection, MidiOutputConnection) {
  let out = midi_output("CommandPad DAW Output");
  let (daw_port, midi_port) = get_midi_out_ports(&out);

  let daw_conn = out.connect(&daw_port, "commandpad-daw-out").unwrap();

  let midi_conn = {
    let midi_out = midi_output("CommandPad MIDI Output");

    midi_out.connect(&midi_port, "commandpad-midi-out").unwrap()
  };

  (daw_conn, midi_conn)
}

impl Launchpad {
  pub fn new() -> Launchpad {
    let (daw_conn, midi_conn) = initialize_output();

    Launchpad {
      color_map: empty_pad_state(),
      daw_conn,
      midi_conn,
      midi_in: None,
    }
  }

  pub fn send(&mut self, message: &[u8]) {
    self.midi_conn.send(message).unwrap();
  }

  pub fn send_daw(&mut self, message: &[u8]) {
    self.daw_conn.send(message).unwrap();
  }

  pub fn set_programmer_mode(&mut self, is_enabled: bool) {
    let mode = if is_enabled { 1 } else { 0 };

    self.send(&[240, 0, 32, 41, 2, 12, 14, mode, 247]);
  }

  pub fn light_on(&mut self, position: u8, color: u8) {
    self.color_map.insert(position, color);

    self.send(&[0b10010000, position, color]);
  }

  pub fn paint_static_grid(&mut self, grid: Vec<Vec<u8>>) {
    let mut specs: Vec<u8> = vec![240, 0, 32, 41, 2, 12, 3];

    for (y, row) in grid.into_iter().enumerate() {
      for (x, col) in row.into_iter().enumerate() {
        let note = position_to_note(x as u8, y as u8);

        specs.append(&mut vec![0, note, col]);
      }
    }

    specs.push(247);
    self.send(&specs);
  }

  pub fn paint_rgb_grid(&mut self, grid: Vec<Vec<Vec<u8>>>) {
    let mut specs: Vec<u8> = vec![240, 0, 32, 41, 2, 12, 3];

    for (y, row) in grid.into_iter().enumerate() {
      for (x, col) in row.into_iter().enumerate() {
        let note = position_to_note(x as u8, y as u8);
        let mut spec = vec![3, note];

        let mut colcl = col.clone();
        spec.append(&mut colcl);
        specs.append(&mut spec);
      }
    }

    specs.push(247);
    self.send(&specs);
  }

  pub fn cycle_color(&mut self, position: u8) {
    let mut color = self.color_map.get(&position).unwrap_or(&0);
    if color > &127 {
      color = &0
    }

    self.light_on(position, color + 1);
  }

  pub fn setup(&mut self) {
    // Enable Programmer Mode
    self.set_programmer_mode(true);
    println!("programmer mode enabled");
  }

  pub fn clear(&mut self, color: u8) {
    for position in 11..100 {
      self.light_on(position, color);
    }
  }
}

pub fn rand_u8() -> u8 {
  let mut rng = rand::thread_rng();

  rng.gen_range(80..87)
}

pub fn blank_rgb_canvas() -> Vec<Vec<Vec<u8>>> {
  (0..8)
    .map(|_| (0..8).map(|_| vec![0, 0, 0]).collect())
    .collect()
}
