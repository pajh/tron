extern crate rand;

mod board;

mod support;

mod aimonte;

use aimonte::MonteAI;

use std::io;

mod ai;

use ai::AI;
use std::time::{Duration, Instant};

//mod floodfill;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap() 
    };
}

fn main() {
    let mut ai = MonteAI::new();
    
    let mut start_pos: [(usize, usize); 4] = [(0, 0); 4];
    let mut game_turn = 1;
    // game loop

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let start_of_loop = Instant::now();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n = parse_input!(inputs[0], usize); // total number of players (2 to 4).
        let me = parse_input!(inputs[1], usize); // your player number (0 to 3).

        let mut cur_pos = [(0, 0); 4];
        for i in 0..n as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x0 = parse_input!(inputs[0], isize); // starting X coordinate of lightcycle (or -1)
            let y0 = parse_input!(inputs[1], isize); // starting Y coordinate of lightcycle (or -1)
            let x1 = parse_input!(inputs[2], isize); // starting X coordinate of lightcycle (can be the same as X0 if you play before this player)
            let y1 = parse_input!(inputs[3], isize); // starting Y coordinate of lightcycle (can be the same as Y0 if you play before this player)

            let px: usize = if x1 < 0 { 999 } else { x1 as usize };
            let py: usize = if y1 < 0 { 999 } else { y1 as usize };
            cur_pos[i] = (px, py);

            if game_turn == 1 && x0 != -1 && y0 != -1 {
                start_pos[i] = (x0 as usize, y0 as usize);
            }
        }
        if game_turn == 1 {
            ai.start(&start_pos, n, me, false);
        }

        let mv = ai.next_move(&cur_pos, start_of_loop);
        println!("{}",mv.to_string());
        game_turn += 1;
    }
}