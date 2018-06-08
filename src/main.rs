/* Quarter the board
   Make me always player 0 - fix order of other players
   stop giving me priority when I have just made a move
*/
extern crate rand;
use rand::distributions::{Distribution, Uniform};

mod board;
use board::Board;
use board::MoveTree;

mod support;
use support::PPos;
use support::PlayerData;

mod aiminmax;
use aiminmax::MinMaxAI;

mod ailong;
use ailong::LongAI;

mod aimonte;
use aimonte::MonteAI;

use std::io;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
mod simgame;
use simgame::Game;
mod aiflood;
use aiflood::FloodAI;
mod ai;
mod floodfill;
mod neural;
use neural::NeuralAI;
use neural::NeuralNet;
const c_def:&str = "\x1b[m";
const c_red:&str = "\x1b[31m";
const c_grn:&str = "\x1b[32m";

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn board_load(b: &mut Board, pdata: &mut PlayerData) {
    let f = File::open("map.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut line = String::new();
    let len = file.read_line(&mut line);
    let mut split = line.trim().split(" ");
    let nop = split.next().unwrap().trim().parse::<usize>().unwrap();
    pdata.no_players = nop;

    for p in 0..nop {
        let x = split.next().unwrap().trim().parse::<usize>().unwrap();
        let y = split.next().unwrap().trim().parse::<usize>().unwrap();
        pdata.players[p].pos = (x, y);
    }

    let mut y = 0;
    for line in file.lines() {
        let l = line.unwrap();
        let ts = l.trim();
        println!("{}", ts);
        let mut x = 0;
        for c in ts.chars() {
            match c {
                '0' => b.set((x, y), 1, 0),
                '1' => b.set((x, y), 1, 1),
                '2' => b.set((x, y), 1, 2),
                '3' => b.set((x, y), 1, 3),
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }

    // print_turn(pdata, b);
}

fn time_floodfill() {
    let mut board = Board::new();
    let mut players: PlayerData = PlayerData::new(3);

    board.set((1, 1), 1, 1);
    players.players[1].pos = (1, 1);

    board.set((28, 1), 1, 2);
    players.players[2].pos = (28, 1);

    for i in 0..11 {
        board.set((8, 19 - i), 1, 0);
        board.set((15, i), 1, 1);
        board.set((23, 19 - i), 1, 2);
    }
    use std::time::Instant;

    let start = Instant::now();

    // do stuff
    let empty_tree = MoveTree::new();
    for yb in 0..18 {
        for x in 0..29 {
            let y = 19 - yb;
            board.set((x, y), 1, 0);
            players.players[0].pos = (x, y);
            //            let s = floodfill::flood_fill2(&board, &players, &empty_tree, 0);
            //println!("({},{})={:?}",x,y,s);
        }
    }
    let elapsed = start.elapsed();
    // debug format:
    let c = elapsed.subsec_nanos() as f64 / 1000000000f64;
    println!("elapsed : {:}", c);

    //flood_fill(board: &Board, pdata: &PlayerData, nt:Option<(&MoveTree, usize)>) -> PScore
}

fn is_in(p: PPos, a: [PPos; 4]) -> bool {
    for i in 0..4 {
        if p == a[i] {
            return true;
        }
    }
    false
}

fn gen_rand_starts(rng: &mut rand::ThreadRng) -> [PPos; 4] {
    let w_range = Uniform::new_inclusive(0, 29);
    let h_range = Uniform::new_inclusive(0, 19);
    let pp: PPos = (99, 99);
    let mut spos = [pp; 4];
    for i in 0..4 {
        loop {
            let np: PPos = (w_range.sample(rng), h_range.sample(rng));
            if !is_in(np, spos) {
                spos[i] = np;
                break;
            }
        }
    }

    spos
}

fn play_game() {
    /*    let mut file = OpenOptions::new().append(true).create(true).open("testfile.txt").unwrap();   
        let s = "hello\n there".to_string(); 
        file.write_all(s.as_bytes()).expect("Unable to write");;
        return;*/

    
    let mut rng = rand::thread_rng();

    let mut ai0 = NeuralAI::new();
    let mut ai1 = NeuralAI::new();
    let mut ai2 = FloodAI::new();
    let mut ai3 = LongAI::new();
    let mut ai4 = MonteAI::new();
    

    let mut wins: [usize; 4] = [0, 0, 0, 0];
    let mut improves = 0;
    let mut ai_to_dump:Option<usize> = None;

    const NUM_ROUNDS:usize = 100;

    for i in 0..NUM_ROUNDS {

        // Borrow fighting, save the ai from the previous game's move if needed.
        //if let Some(ain) = ai_to_dump {
        //    if ain == 0 { ai0.dump_moves(); } else {ai1.dump_moves(); }
        //    ai_to_dump = None;
        //    improves += 1;
        //}    

        let spos = gen_rand_starts(&mut rng);

        //let mut rndai = 99;
        
        let mut game = Game::new();
        game.add_player(&mut ai0, spos[0]);
        game.add_player(&mut ai1, spos[1]);
        game.add_player(&mut ai2, spos[2]); 
        game.add_player(&mut ai4, spos[3]);

        let (result1, winner1) = game.play();
        wins[winner1] += 1;
        eprintln!(" {} GAME:[{} {} {} {}] winner {}", i,result1[0], result1[1],result1[2], result1[3], winner1);
/*        rndai = if result1[0] < result1[1] { 1 } else { 0 };   // compare where the neural nets finished I want the worst one
            
        let (result2, winner2) = game.replay(rndai);  // This tells game to turn on random moves for this ai

        if rndai == 0 && result2[0] < result1[0] {
            ai_to_dump = Some(0);
        }

        if rndai == 1 && result2[1] < result1[1] {
            ai_to_dump = Some(1);
        }
        
        let r0_c = if result2[0] < result1[0] && rndai == 0 { c_grn } else { "" };
        let r1_c = if result2[1] < result1[1] && rndai == 1 { c_grn } else { "" };
        eprintln!("REPLAY [{}{}{} {}{}{} {} {}] winner {}",r0_c,result2[0],c_def,r1_c,result2[1],c_def,result2[2], result2[3], winner2);        */
    }
    let mut wrate:[f32;4] = [0.0;4];
    for i in 0..4  {
        wrate[i] = 100.0 * wins[i] as f32 / NUM_ROUNDS as f32;
    }
    let ip = improves as f32 * 100.0 / NUM_ROUNDS as f32;
    eprintln!("WINRATES nn0={:.1}% nn1={:.1}% flood={:.1}% long={:.1}%, games played {}, imp {:.1}%",wrate[0],wrate[1],wrate[2],wrate[3],NUM_ROUNDS, ip);
}

fn test_nn() {
    let mut nn = NeuralNet::new();
    nn.load();
    neural::process_data(nn);
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    //let mut board = Board::new(false);
    //let mut players: PlayerData = PlayerData::new(0, 0);
    //let mut ai = MinMaxAI::new();
    //let mut ai1 = LongAI::new();
    let mut start_pos: [(usize, usize); 4] = [(0, 0); 4];
    let mut game_turn = 1;
    let local = true;
    // game loop

    if local {
        //test_nn();
        play_game();
        return;
    }

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
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

        game_turn += 1;
    }
}
