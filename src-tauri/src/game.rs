use crate::flappy::Flappy;
use crate::launchpad::{blank_rgb_canvas, get_midi_in_ports, midi_input, rand_u8, Launchpad};

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub fn start_game() {
  let mut launchpad = Launchpad::new();
  launchpad.setup();

  let mut rng = rand::thread_rng();

  launchpad.clear(1);

  let fps = 6;

  let mut flappy = Flappy::new();

  let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

  let input = midi_input("CommandPad MIDI Input");
  let (_, midi_port) = get_midi_in_ports(&input);

  let midi_in = input
    .connect(
      &midi_port,
      "CommandPad MIDI Input",
      move |ms, message, _extra| {
        if let &[command, position, velocity] = message {
          if command == 144 && velocity > 0 {
            tx.send(100).unwrap();
          }
        }
      },
      (),
    )
    .unwrap();

  loop {
    if flappy.is_dead {
      launchpad.clear(4);
      println!("Game Over! Your Score: {}", flappy.score);

      loop {
        let v = rx.recv().unwrap_or(-1);
        if v == 100 {
          println!("restarting...");
          flappy.reset();

          break;
        }
      }
    }

    let v = rx.try_recv().unwrap_or(-1);
    flappy.jumping = v == 100;
    flappy.tick();

    let mut canvas = blank_rgb_canvas();

    for (time, top_h, bottom_h) in flappy.obstacles.clone() {
      if flappy.is_dead {
        break;
      }

      if time < 1 {
        continue;
      }

      for y in 0..top_h {
        if time == 1 && y == flappy.char_y as u8 {
          println!("colliding at top!");
          flappy.is_dead = true;
        }

        canvas[y as usize][(time - 1) as usize] = vec![0, 127, 0];
      }

      for y in (9 - bottom_h)..9 {
        if time == 1 && y - 1 == flappy.char_y as u8 {
          println!("colliding at bottom!");
          flappy.is_dead = true;
        }

        canvas[(y - 1) as usize][(time - 1) as usize] = vec![127, 0, 0];
      }
    }

    canvas[flappy.char_y as usize][1] = if flappy.jumping {
      vec![0, 0, 127]
    } else {
      vec![127, 0, 127]
    };

    launchpad.paint_rgb_grid(canvas);

    sleep(Duration::from_millis(1000 / fps))
  }
}
