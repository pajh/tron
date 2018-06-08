use board::AvailMoves;
use board::BHEIGHT;
use board::BWIDTH;
use board::Board;
use board::MoveTree;
use std::io;
use support::Fill;
use support::MAXPLAYERS;
use support::Move;
use support::PPos;
use support::PScore;
use support::Player;

use std::collections::VecDeque;

pub struct FloodBoard {
    pub cells: [u16; 600],
    pub play: [u8; 600],
    pub p_pos: [Player; MAXPLAYERS],
    no_players: usize,

}

impl FloodBoard {
    pub fn new(board: &Board, players: [Player; MAXPLAYERS], no_players:usize ) -> Self {
        //if no_players > 2  { panic!("No players > 2")};
        let mut new_board = FloodBoard {
            cells: [32000; 600],
            play: [MAXPLAYERS as u8; 600],
            p_pos: players,
            no_players: no_players,

        };
        for i in 0..600 {
            if board.cells[i] != 0 && new_board.p_pos[board.play[i] as usize].dead == false {
                new_board.cells[i] = 0;
            }
        }
        new_board
    }

    pub fn blank_tree(mut self, tree: &MoveTree, node: usize) -> Self {
        let mut cnode = node;
        while cnode != 0 {
            let (pos, play, parent) = tree.get(cnode);
            if self.p_pos[play as usize].dead == false {
                self.cells[Board::pos(pos)] = 0;
            }
            cnode = parent;
        }
        self
    }

    pub fn setp(&mut self, f: &Fill) {
        let ps = Board::pos(f.pos);
        self.cells[ps] = f.depth;
        self.play[ps] = f.player;
    }

    pub fn set(&mut self, pos: PPos, v: u16, p: u8) {
        let ps = Board::pos(pos);
        self.cells[ps] = v;
        self.play[ps] = p;
    }

    pub fn get(&self, pos: PPos) -> (u16, u8) {
        (self.cells[Board::pos(pos)], self.play[Board::pos(pos)])
    }

    pub fn score_board(&self) -> PScore {
        let mut scores = [0; MAXPLAYERS];
        for i in 0..600 {
            if self.play[i] < MAXPLAYERS as u8 {
                scores[self.play[i] as usize] = scores[self.play[i] as usize] + 1;
            }
        }
        scores
    }

    pub fn to_string(&self, perspect: u8) -> String {
        let mut max_depths: [u16; MAXPLAYERS] = [0; MAXPLAYERS];
        for i in 0..600 {
            let v = self.cells[i];
            let play = self.play[i] as usize;
            if play < MAXPLAYERS {
                if v > max_depths[play] {
                    max_depths[play] = v;
                }
            }
        }
        let mut s = "".to_string();
        for y in 0..BHEIGHT {
            for x in 0..BWIDTH {
                let (v, p) = self.get((x, y));

                if p == MAXPLAYERS as u8 {
                    s.push_str("0,");
                } else {
                    let cval = (max_depths[p as usize] + 1 - v) as isize;
                    if p == perspect {
                        s.push_str(&format!("{},", cval));
                    } else {
                        s.push_str(&format!("{},", 0-cval ));
                    }
                }
            }
            s.push_str("\n");
        }
        s
    }

    pub fn avail_moves(&self, p: PPos) -> AvailMoves {
        let mut av = AvailMoves::new();
        if p.0 > 0 && self.get((p.0 - 1, p.1)).1 > 0 {
            av.add(Move::L((p.0 - 1, p.1)));
        }

        if p.0 < BWIDTH - 1 && self.get((p.0 + 1, p.1)).1 > 0 {
            av.add(Move::R((p.0 + 1, p.1)));
        }

        if p.1 > 0 && self.get((p.0, p.1 - 1)).1 > 0 {
            av.add(Move::U((p.0, p.1 - 1)));
        }

        if p.1 < BHEIGHT - 1 && self.get((p.0, p.1 + 1)).1 > 0 {
            av.add(Move::D((p.0, p.1 + 1)));
        }
        av
    }

    fn calc_move_priority(last_player:usize) -> [usize;4] {

        let mut prio = [0;4];        
        let mut c_play = last_player;
        for c_prio in 0..4 {
            prio[c_play] = c_prio;
            c_play = (c_play + 3 ) % 4;
        }
        prio
    }

 
    pub fn flood_fill(&mut self, last_player: u8) -> PScore {
        if self.p_pos[0].dead {
            panic!("Why am I dead?");
        }
        let mut queue: VecDeque<Fill> = VecDeque::new();
        let p_prio = FloodBoard::calc_move_priority(last_player as usize);

        for p in self.p_pos[0..self.no_players].iter() {
            if p.dead { continue; }
            for mv in self.avail_moves(p.pos).iter() {
                queue.push_back(Fill {
                    depth: 1,
                    pos: mv.get_point(),
                    player: p.no as u8,
                });
            }
        }
        
        while queue.len() > 0 {
          
            let f: Fill = queue.pop_front().unwrap();

            let (b_val, b_play) = self.get(f.pos);
            
            if ( f.depth < b_val ) || ( f.depth == b_val && p_prio[f.player as usize] > p_prio[b_play as usize] ) {
                self.setp(&f);
                let new_depth = f.depth + 1;
                let t_m = AvailMoves::calc(f.pos);

                for mv in t_m.iter() {
                    let (b_val, b_play) = self.get( mv.get_point() );
                    if ( new_depth < b_val ) || ( new_depth == b_val && p_prio[f.player as usize] > p_prio[b_play as usize] ) {
                        queue.push_back(Fill {
                            depth: new_depth,
                            pos: mv.get_point(),
                            player: f.player,
                        });
                    }
                }
            }
        }
        let mut pscore = self.score_board();
        if pscore[0] == 0 && pscore[1] == 0 && pscore[2] == 0 && pscore[3] == 0 {
            pscore[last_player as usize] = 250;
            //eprintln!("*******SCORED ZERO**************");
            //self.print();
        }
        if pscore[0] == 0 && pscore[1] == 0 && pscore[2] == 0 && pscore[3] == 0 {         
            eprintln!("*******SCORED ZERO**************");
            self.print();
        }

        pscore
    }

    fn is_player_here(&self, pos:PPos) -> usize {
        for i in 0..4 {
            if self.p_pos[i].dead == false && self.p_pos[i].pos == pos {
                return i;
            }
        }
        MAXPLAYERS
    } 

    pub fn print(&self) {
        for y in 0..BHEIGHT {
            for x in 0..BWIDTH {
                let pp = self.is_player_here((x,y));
                if  pp != MAXPLAYERS {
                    eprint!("\x1b[31m{}\x1b[m",pp);
                    continue;
                }
                let (v,p) = self.get((x,y));
                if p == MAXPLAYERS as u8 {
                    eprint!(" ");
                } else {
                    eprint!("{}",p);
                }
            }
            eprintln!("");
        }
    }
}
