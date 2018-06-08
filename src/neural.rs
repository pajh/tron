use ai::AI;
use board::AvailMoves;
use board::Board;
use floodfill::FloodBoard;
use rand::Rng;
use rand::ThreadRng;
//use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform};
use rand::thread_rng;
use std;
use std::fs::File;
use std::io::{BufRead, BufReader};
use support::MAXPLAYERS;
use support::Move;
use support::PPos;
use support::PlayerData;
use support::append_str_to_file;//(filename, data)
use aiminmax::scan_move;
use aiminmax::score_for_player;//(scores, player)
use board::MoveTree;
use std::time::{Duration, Instant};

pub const CELL_NORMAL: f32 = 100.0;
pub const SCORE_NORMAL: f32 = 500.0;
pub const INPUT_SIZE: usize = 600;

pub fn encode_board(board: &FloodBoard, player: usize) -> String {
    
    let mut v: Vec<f32> = Vec::new();
    weigh_board(board, player, &mut v);
    vec2str(&v)
}

fn vec2str(v:&Vec<f32>)->String {
    let mut s = String::new();
    for f in v {
        s.push_str(&format!("{},", f));
    }    
    s
}

pub fn 
weigh_board(board: &FloodBoard, player: usize, vin: &mut Vec<f32>) {
    vin.clear();
    let p_pos = board.p_pos[player].pos;

    let h_flip = if p_pos.0 < 15 { false } else { true };
    let v_flip = if p_pos.1 < 10 { false } else { true };

    let mut max_depths: [u16; MAXPLAYERS] = [0; MAXPLAYERS];
    for i in 0..600 {
        let v = board.cells[i];
        let play = board.play[i] as usize;
        if play < MAXPLAYERS {
            if v > max_depths[play] {
                max_depths[play] = v;
            }
        }
    }

    for y in 0..::board::BHEIGHT {
        for x in 0..::board::BWIDTH {
            let f_x = if h_flip { ::board::BWIDTH - 1 - x } else { x };
            let f_y = if v_flip { ::board::BHEIGHT - 1 - y } else { y };
            let (v, p) = board.get((f_x, f_y));
            let mut cval = 0f32;
            if p < MAXPLAYERS as u8 {
                cval = scale_1((max_depths[p as usize] + 1 - v) as f32, CELL_NORMAL);
                if p != player as u8 {
                    cval = 0.0 - cval;
                }
            }
            vin.push(cval);
        }
    }
}

pub fn scale_1(n: f32, s: f32) -> f32 {
    (n / s).min(1.0).max(-1.0)
}

fn sigmoid(y: f32) -> f32 {
    1f32 / (1f32 + (-y).exp())
}

fn relu(y:f32) -> f32 {
    y.max(0.0)
}

fn loadvec<T: std::str::FromStr>(v: &mut Vec<T>, name: &str, expect:usize) {
    let file = File::open(name).unwrap();
    for line in BufReader::new(file).lines() {
        let s = line.unwrap();
        let opt = s.parse::<T>();
        if let Ok(r) = opt {
            v.push(r);
        }
    }
    if expect != v.len()  {
        panic!("Error loading from file {}, expected {} but found {}",name, expect, v.len());
    }
}

fn loadvec_from_str<T: std::str::FromStr>(v: &mut Vec<T>, data: &str, delimiter: char) {
    v.clear();
    let inputs = data.split(delimiter).collect::<Vec<_>>();
    for i in inputs {
        let opt = i.parse::<T>();
        if let Ok(r) = opt {
            v.push(r);
        }
    }
}

pub fn process_data(nn: NeuralNet) {
    let mut v: Vec<f32> = Vec::new();
    let file = File::open("append.txt").unwrap();
    let mut count = 0;
    for line in BufReader::new(file).lines() {
        let s = line.unwrap();
        if count > 20 {
            break;
        }
        loadvec_from_str(&mut v, &s, ',');
        let res = nn.calc(&v);
        eprintln!("#{}={}", count, res);
        count += 1;
    }
}

pub struct NeuralNet {
    weights1: Vec<f32>,
    bias1: Vec<f32>,
    weights2: Vec<f32>,
    bias2: Vec<f32>,
    weights3: Vec<f32>,
    bias3: Vec<f32>,        
}

const LYR_INPUT:usize = 600;
const LYR_H1:usize = 12;
const LYR_H2:usize = 12;
const LYR_OUT:usize = 1;

impl NeuralNet {
    pub fn new() -> Self {
        NeuralNet {
            weights1: Vec::with_capacity(LYR_INPUT * LYR_H1),
            bias1: Vec::with_capacity(LYR_H1),
            weights2: Vec::new(),
            bias2: Vec::new(),
            weights3: Vec::new(),
            bias3: Vec::new(),            
        }
    }

    pub fn load(&mut self) {
        loadvec(&mut self.weights1, "nnweight1.txt", LYR_INPUT * LYR_H1 );
        loadvec(&mut self.weights2, "nnweight2.txt", LYR_H1 * LYR_H2 );
        loadvec(&mut self.weights3, "nnweight3.txt", LYR_H2 * LYR_OUT );

        loadvec(&mut self.bias1, "nnbias1.txt", LYR_H1 );
        loadvec(&mut self.bias2, "nnbias2.txt", LYR_H2);
        loadvec(&mut self.bias3, "nnbias3.txt", LYR_OUT);

        eprintln!(
            "Vector sizes w1 {} w2 {} b1 {} b2 {}",
            self.weights1.len(),
            self.weights2.len(),
            self.bias1.len(),
            self.bias2.len()
        );
    }

    pub fn calc(&self, input: &Vec<f32>) -> f32 {
        if input.len() < LYR_INPUT {
            panic!("Input length is wrong {}", input.len());
        }

        let mut r1 = [0.0; LYR_H1];

        for i in 0..r1.len() {
            let mut temp: f32 = 0.0;
            for j in 0..LYR_INPUT {
                let apos = j * LYR_H1 + i; // Might be other way around
                temp += self.weights1[apos] * input[j];
            }
            temp += self.bias1[i];
            r1[i] = relu(temp);
        }

        let mut r2 = [0.0;LYR_H2];

        for i in 0..r2.len() {
            let mut temp: f32 = 0.0;
            for j in 0..LYR_H1 {
                let apos = j * LYR_H2 + i; // Might be other way around
                temp += self.weights2[apos] * r1[j];
            }
            temp += self.bias2[i];
            r2[i] = relu(temp);
        }

        let mut temp: f32 = 0.0;
        for i in 0..r2.len() {
            temp += r2[i] * self.weights3[i];
        }

        temp += self.bias3[0];

        temp
    }
}

pub struct NeuralAI {
    board: Board,
    pdata: PlayerData,
    pmap: [usize; 4], // Used to map 'games' player number to my I am always '0'
    nn: NeuralNet,
    inv: [Vec<f32>; 4],
    rand: bool,
    rng: ThreadRng,
    game_id: usize,
    better_moves: String,
    tree: MoveTree, // Only used for evaluating random positions
    rand_mv_made: bool,
}

impl NeuralAI {
    pub fn new() -> Self {
        let board = Board::new();

        let pdata = PlayerData::new(0);
        let mut nn = NeuralNet::new();
        nn.load();
        let nai = NeuralAI {
            board: board,
            pdata: pdata,
            pmap: [0; 4],
            nn: nn,
            inv: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            rand: false,
            rng: thread_rng(),
            game_id:99,
            better_moves: String::new(),
            tree: MoveTree::new(),
            rand_mv_made: false,
        };

        nai
    }
    pub fn dump_moves(&self) {
        append_str_to_file("current.txt", &self.better_moves);
    }    
}

impl AI for NeuralAI {
    fn next_move(&mut self, p_pos: &[PPos; 4], start_time:Instant) -> Move {
        AI::handle_others(&mut self.pdata, p_pos, self.pmap, &mut self.board);

        let mut avail_moves = AvailMoves::calc_board(&self.board, self.pdata.me().pos);

        if avail_moves.len() == 0 {
            return Move::U((0, 0)); // Dummy suicide move
        }

        let mut best_option = 5;
        let mut move_scores = [0f32, 0f32, 0f32, 0f32];

        if avail_moves.len() > 1 {
            for i in 0..avail_moves.len() {
                let p = avail_moves.moves[i].get_point();
                let mut pd_n = self.pdata.players;
                pd_n[0].pos = p;
                let mut fb = FloodBoard::new(&self.board, pd_n, self.pdata.no_players);
                fb.set(p, 0, ::support::MAXPLAYERS as u8);
                fb.flood_fill(0u8);
                weigh_board(&fb, 0usize, &mut self.inv[i]);
                move_scores[i] = self.nn.calc(&self.inv[i]);
            }

            let mut best_value = -1000f32;
            for i in 0..avail_moves.len() {
                if move_scores[i] > best_value {
                    best_value = move_scores[i];
                    best_option = i;
                }
            }

            if !self.rand_mv_made && self.rand && self.rng.gen::<f32>() < 0.01f32 {
                // Hardcoded 1% chance

                let mut second_best = -1000f32;
                let mut second_option = 99;
                for i in 0..avail_moves.len() {
                    if i == best_option { continue; }
                    if move_scores[i] > second_best {
                        second_best = move_scores[i];
                        second_option = i;
                    }
                }
                //eprintln!("Move {} score {} and move {} score {}",best_option,best_value, second_option, second_best);
                if best_value - second_best < 0.3 { // Todo all the recording of the alternate move
                    let mut this_swap=String::new();
                    self.rand_mv_made = true;
                    
                    let mut this_move = avail_moves.moves[best_option].get_point();
                    //self.tree.clear();

                    //let scn = scan_move(0, this_move, self.pdata, &self.board, &mut self.tree, 0, 0);
                    let score_1:f32 = second_best - 0.1; //score_for_player( scn, 0 ) as f32 / SCORE_NORMAL;

                    this_move = avail_moves.moves[second_option].get_point();
                    //self.tree.clear();
                    let score_2:f32 = best_value + 0.1;// score_for_player( scan_move(0, this_move, self.pdata, &self.board, &mut self.tree, 0, 0), 0 ) as f32 / SCORE_NORMAL;

                    //println!("Score comparison MINMAX[{} {}] NN[{} {}] {}", score_1, score_2, best_value, second_best, score_2 > score_1);
  
                    this_swap.push_str(&vec2str(&self.inv[best_option] ));
                    this_swap.push_str(&format!("{}\n",score_1));
                    this_swap.push_str(&vec2str(&self.inv[second_option] ));
                    this_swap.push_str(&format!("{}\n",score_2 ));
                    self.better_moves.push_str(&this_swap);                    
                    best_option = second_option;
                   // eprintln!("{} Made a random move", self.game_id);
                }
            }
        } else {
            best_option = 0; // Only one move
        }

        match self.board
            .is_valid_move(self.pdata.players[0].pos, avail_moves.moves[best_option])
        {
            None => eprintln!(
                "My best move is not valid {:?} {}",
                self.pdata.players[0].pos,
                avail_moves.moves[best_option].to_debug()
            ),
            Some(new_pos) => {
                self.pdata.players[0].pos = new_pos;
                self.board.set(new_pos, 1, 0 as u8);
            }
        }

        avail_moves.moves[best_option]
    }

    fn start(&mut self, start_pos: &[PPos; 4], no_players: usize, you: usize, play_rand:bool) {
        self.pdata = PlayerData::new(no_players);
        self.board.clear();
        self.rand = play_rand;
        self.game_id = you;
        self.better_moves.clear();
        self.rand_mv_made = false;

        for i in 0..4 {
            self.pmap[i] = (i + 4 - you) % 4;
        }

        self.pdata.no_players = no_players;
        for i in 0..no_players {
            self.pdata.players[self.pmap[i]].pos = start_pos[i];
            self.board.set(start_pos[i], 1, self.pmap[i] as u8);
        }
    }
}
