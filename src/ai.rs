use board::Board;
use support::Move;
use support::PPos;
use support::PScore;
use support::PlayerData;
use std::time::{Duration, Instant};

pub trait AI {
    fn next_move(&mut self, players: &[PPos; 4], start_time:Instant) -> Move;
    fn start(&mut self, start_pos: &[PPos; 4], no_players: usize, you: usize,play_rand:bool);
}

impl AI {
    // Get the other player state from the game
    // perform a mapping from its player number system to ours (we are alwats p0)
    //
    pub fn handle_others(
        pdata: &mut PlayerData,
        ppos: &[PPos; 4],
        pmap: [usize; 4],
        board: &mut Board,
    ) {
        for i in 0..pdata.no_players {
            let map_pos = pmap[i];
            let play = &mut pdata.players[map_pos];

            if play.dead {
                continue;
            }

            if map_pos == 0 {
                // sanity check that I am where I think I am
                if ppos[i] != play.pos {
                    panic!("I am not where I expected to be :(");
                }
            } else {
                // Other player
                //let t_p = ppos[i];
                if ppos[i].1 == 999 {
                    // they died
                    play.dead = true;
                    board.clear_player_from_board(map_pos as u8);
                } else {
                    play.pos = ppos[i];
                    board.set(ppos[i], 1, map_pos as u8);
                }
            }
        }
    }

    pub fn score_for_player(scores: PScore, player: usize) -> i16 {
        let mut score = 0i16;
        for i in 0..::support::MAXPLAYERS {
            if player == i {
                score += scores[i];
            } else {
                score -= scores[i];
            }
        }
        score
    }
}
