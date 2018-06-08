use ai::AI;
use support::PPos;
use support::PlayerData;
use board::Board;
use std::time::{Duration, Instant};

pub struct Game<'a> {
    playai: [Option<&'a mut AI>;4],
    playid: [usize;4],
    dead: [bool;4],
    finishpos: [usize;4],
    pos:[PPos;4],
    spos:[PPos;4],
    noplayers: usize,
    alive: usize,
}

impl<'a> Game<'a> {
    pub fn new()->Self {
        Game {playai:[None,None,None,None], dead:[true;4], finishpos:[0;4], noplayers:0, playid:[0;4], pos:[(99,99),(99,99),(99,99),(99,99)], alive:0, spos:[(99,99),(99,99),(99,99),(99,99)]}
    }

    pub fn add_player<T:AI>(&mut self, ai:&'a mut T, pos:PPos) {
        
        if self.noplayers == 4 { panic!("Too many players")};
        self.playid[self.noplayers] = self.noplayers;
        self.playai[self.noplayers] = Some(ai);
        self.dead[self.noplayers] = false;
        self.pos[self.noplayers] = pos;
        self.noplayers += 1;
        self.alive += 1;
        
    }

    fn kill_player(&mut self, id:usize) {
        if id >= self.noplayers {
            panic!("Invalid player")
        }

        if self.dead[id] {
            panic!("Player already dead");
        }

        self.dead[id] = true;
        self.pos[id] = (999,999);
        self.finishpos[id] = self.alive;
        self.alive -= 1;
        
    }

    fn next_live(&self, player:usize)->usize {
        let mut next_player = (player + 1) % self.noplayers;
        if self.dead[next_player] {
            next_player = self.next_live(next_player);
        }
        next_player
    }

    pub fn replay(&mut self, random_player:usize)->([usize;4],usize) {
        self.alive = self.noplayers;
        self.pos = self.spos;

        for i in 0..self.noplayers {
            self.dead[i] = false;
            self.finishpos[i] = 0;            
        }
        self.run(random_player)
    }

    pub fn play(&mut self)->([usize;4],usize) {
        self.spos = self.pos; // copy start positions for replay
        self.run(99)
    }

    pub fn run(&mut self, random_player:usize)->([usize;4],usize) {
        if self.noplayers < 2 { panic!("Less than 2 players")};
        self.spos = self.pos; // copy start positions for replay

        let mut board=Board::new();
        for cp in 0..self.noplayers {           // Send all players a start message.
            board.set(self.pos[cp], 1, cp as u8);
            match self.playai[cp] {
                Some(ref mut ai) => {
                    
                    ai.start(&self.pos, self.noplayers, cp, cp==random_player);
                    
                }
                _=>{ panic!("No AI found")},
            }            
        }

        let mut cplayer = 0; // always goes first
        let mut turn = 0;
        while self.alive > 1 {
            turn+=1;
           //print_turn(self.pos, &board); 
           let mv = match self.playai[cplayer] {                    
                    Some(ref mut ai) => { ai.next_move(&self.pos, Instant::now()) },
                    _=>panic!("ss"),
                };            
            match board.is_valid_move(self.pos[cplayer], mv) {
                None=>{ board.clear_player_from_board(cplayer as u8); 
                        self.kill_player(cplayer) 
                        },
                Some(new_pos)=> {
                    self.pos[cplayer] = new_pos;
                    board.set(new_pos, 1, cplayer as u8);                    
                }
            }

            cplayer = self.next_live(cplayer);

        }

        if self.dead[cplayer] { panic!("Everyone is dead :(");}
        self.finishpos[cplayer] = 1usize; // winnah
        (self.finishpos, cplayer)
    }
}

pub fn print_turn(pos:[PPos;4], b: &Board) {
    eprintln!("---------------------------------");    
    for i in 0..pos.len() {
        eprint!(" {} {}", pos[i].0, pos[i].1,);
    }
    eprintln!("");
    for y in 0..::board::BHEIGHT {
        let mut l = String::new();
        for x in 0..::board::BWIDTH {
            let (v, p) = b.get((x, y));
            if v == 0 {
                l.push('.');
            } else {
                if (x,y) == pos[p as usize] {
                    l.push_str(&format!("\x1b[31m{}\x1b[0m", p));
                } else {
                     l.push_str(&format!("{}", p));
                }
            }
        }
        eprintln!("{}", l);
    } 
}