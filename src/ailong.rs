//
// Simple AI find longest direction and go
//

use board::Board;
use board::AvailMoves;
use support::PlayerData;
use support::Move;
use support::PPos;
use ai::AI;
use std::time::{Duration, Instant};


pub struct LongAI {
    board: Board,
    pdata: PlayerData,
    pmap: [usize; 4], // Used to map 'games' player number to my I am always '0'
}

impl LongAI {
    pub fn new() -> Self {
        LongAI {
            board: Board::new(),
            pdata: PlayerData::new(0),
            pmap:[0;4],
        }
    }
}

impl AI for LongAI {
    fn next_move(&mut self, p_pos: &[PPos; 4],_start:Instant) -> Move {
        //self.handle_others(p_pos);
        AI::handle_others(&mut self.pdata, p_pos, self.pmap, &mut self.board);
        let avail_moves = AvailMoves::calc_board(&self.board, self.pdata.me().pos);
        let me = 0;

        if avail_moves.len() == 0 {
            return Move::U((0, 0)); // Dummy suicide move
        }

        let mut longest = 0;
        let mut best_move: Option<Move> = None;

        for i in 0..avail_moves.len() {
            let mut cmove = avail_moves.moves[i];
            let mut clen = 1;
            loop {
                let next_pos = self.board.is_valid_ret_nxt(cmove);
                match next_pos {
                    None => {
                        break;
                    }
                    Some(m) => {
                        clen += 1;
                        cmove = m;
                    }
                }
            }

            if clen > longest {
                longest = clen;
                best_move = Some(avail_moves.moves[i]);
            }
        }

        let mv = best_move.unwrap();
        match self.board.is_valid_move(self.pdata.players[me].pos, mv) {
            None => {}
            Some(new_pos) => {
                self.pdata.players[me].pos = new_pos;
                self.board.set(new_pos, 1, me as u8);
            }
        }

        mv
    }

    fn start(&mut self, start_pos: &[PPos; 4], no_players: usize, you: usize, _play_rand:bool) {
        self.pdata = PlayerData::new(no_players);
        self.board.clear();

        for i in 0..4 {
            self.pmap[i] = (i + 4 - you) % 4;
        }

        self.pdata.no_players = no_players;
        
        for i in 0..no_players {
            self.pdata.players[self.pmap[i]].pos = start_pos[i];
            self.board.set(start_pos[i], 1, self.pmap[i] as u8);
            //eprint!("{}-{:?}", i, self.pdata.players[i].pos);
        }
    }
}
