use ai::AI;
use board::AvailMoves;
use board::Board;
use floodfill::FloodBoard;
use support::Move;
use support::PPos;
use support::PlayerData;
use std::time::{Duration, Instant};

pub struct FloodAI {    
    board: Board,
    pdata: PlayerData,
    pmap: [usize; 4], // Used to map 'games' player number to my I am always '0'
}

impl FloodAI {
    pub fn new() -> Self {
        let board = Board::new();        

        let pdata = PlayerData::new(0);
        FloodAI {            
            board: board,
            pdata: pdata,
            pmap: [0; 4],
        }
    }
}

impl AI for FloodAI {
    fn next_move(&mut self, p_pos: &[PPos; 4],start:Instant) -> Move {
        AI::handle_others(&mut self.pdata, p_pos, self.pmap, &mut self.board);

        let mut avail_moves = AvailMoves::calc_board(&self.board, self.pdata.me().pos);

        if avail_moves.len() == 0 {
            return Move::U((0, 0)); // Dummy suicide move
        }

        let mut best_option = 5;

        if avail_moves.len() > 1 {
            for i in 0..avail_moves.len() {
                let p = avail_moves.moves[i].get_point();
                let mut pd_n = self.pdata.players;
                pd_n[0].pos = p;
                let mut fb = FloodBoard::new(&self.board, pd_n, self.pdata.no_players);
                fb.set(p, 0, ::support::MAXPLAYERS as u8);
                let l_score = fb.flood_fill(0u8);
                avail_moves.score[i] = AI::score_for_player(l_score, 0) as i16;
            }

            let mut best_value = -16000;
            for i in 0..avail_moves.len() {
                if avail_moves.score[i] > best_value {
                    best_value = avail_moves.score[i];
                    best_option = i;
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
        }
    }
}
