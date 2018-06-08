use ai::AI;
use board::AvailMoves;
use board::Board;
use board::MoveTree;
use floodfill::FloodBoard;
use neural::encode_board;
use neural::scale_1;
use support::Move;
use support::PPos;
use support::PScore;
use support::PlayerData;
use support::append_str_to_file;
use std::time::{Duration, Instant};

const MAX_DEPTH: usize = 10;

pub struct MinMaxAI {
    trees: [MoveTree; 4],
    board: Board,
    pdata: PlayerData,
    pmap: [usize; 4], // Used to map 'games' player number to my I am always '0'
}

impl MinMaxAI {
    pub fn new() -> Self {
        let board = Board::new();        
        let pdata = PlayerData::new(0);
        let t0 = MoveTree::new();
        let t1 = MoveTree::new();
        let t2 = MoveTree::new();
        let t3 = MoveTree::new();
        MinMaxAI {
            trees: [t0, t1, t2, t3],
            board: board,
            pdata: pdata,
            pmap: [0; 4],
        }
    }

    fn dump_move(&self, moves: &AvailMoves) -> String {
        let mut s = "".to_string();

        for i in 0..moves.len() {
            let p = moves.moves[i].get_point();
            let mut pd_n = self.pdata.players;
            pd_n[0].pos = p;
            let mut fb = FloodBoard::new(&self.board, pd_n, self.pdata.no_players);
            fb.set(p, 0, ::support::MAXPLAYERS as u8);
            fb.flood_fill(0u8);
            s.push_str(&encode_board(&fb, 0));
            let ml = format!("{}\n", scale_1(f32::from(moves.score[i]), 500f32));
            s.push_str(&ml);
        }
        append_str_to_file("minmaxlog.txt", &s);
        s
    }
}

impl AI for MinMaxAI {
    fn next_move(&mut self, p_pos: &[PPos; 4],  _start:Instant) -> Move {
        AI::handle_others(&mut self.pdata, p_pos, self.pmap, &mut self.board);
        //self.handle_others(p_pos);
        let mut avail_moves = AvailMoves::calc_board(&self.board, self.pdata.me().pos);
        //eprint!("[MINMAX] available moves :");
        avail_moves.print();

        //eprintln!("I am player {}", me);

        if avail_moves.len() == 0 {
            return Move::U((0, 0)); // Dummy suicide move
        }

        let mut best_option = 5;

        if avail_moves.len() > 1 {
            for i in 0..avail_moves.len() {
                self.trees[0].clear();
                let this_move = avail_moves.moves[i].get_point();
                let lscore = scan_move(
                    0,
                    this_move,
                    self.pdata,
                    &self.board,
                    &mut self.trees[0],
                    0,
                    0,
                );
                avail_moves.score[i] = score_for_player(lscore, 0) as i16;
            }

            self.dump_move(&avail_moves);

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

    fn start(&mut self, start_pos: &[PPos; 4], no_players: usize, you: usize, _play_rand: bool) {
        self.pdata = PlayerData::new(no_players);

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

pub fn scan_move(
    player: usize, // This is a move for this player.`
    newmove: PPos,
    mut pdata: PlayerData,
    board: &Board,
    tree: &mut MoveTree,
    node: usize,
    depth: usize,
) -> PScore {
    if depth > MAX_DEPTH {
        return FloodBoard::new(board, pdata.players, pdata.no_players)
            .blank_tree(tree, node)
            .flood_fill(player as u8);
    }

    //sanity_check_move(player, &pdata, newmove, board, tree, node, depth);
    let new_node = tree.add(newmove, node, player as u8);

    pdata.players[player].pos = newmove;

    let mut next_player = pdata.next_live_player(player);
    let mut cplayer = pdata.players[next_player].pos;

    let mut possible_moves = AvailMoves::calc_board_tree_dead(board, tree, new_node, cplayer, &pdata);
    loop {
        if possible_moves.c == 0 {
            pdata.players[next_player].dead = true;

            if pdata.players[0].dead {
                return [-500, 0, 0, 0];
            }

            if pdata.live_count() == 1 {
                return [500, 0, 0, 0];
            }
        }

        next_player = pdata.next_live_player(next_player);
        cplayer = pdata.players[next_player].pos;

        possible_moves = AvailMoves::calc_board_tree_dead(board, tree, new_node, cplayer, &pdata);

        if possible_moves.c > 0 {
            break;
        }
    }

    let mut best_score = -15000i16;
    let mut score_this_move = ::support::ZEROSCORE;
    let mut found = false;
    for i in 0..possible_moves.c {
        let new_move = possible_moves.getxy(i);
        let m_score = scan_move(
            next_player,
            new_move,
            pdata,
            board,
            tree,
            new_node,
            depth + 1,
        );
        if m_score[0] == 0 && m_score[1] == 0 && m_score[2] == 0 && m_score[3] == 0 {
            eprintln!("RECEIVED A Zero move!!!");
        }

        let adj_score = score_for_player(m_score, next_player);

        if adj_score < -14000 {
            eprintln!("LOOK AT THIS SCORE {}", adj_score);
        }

        if adj_score > best_score {
            best_score = adj_score;
            score_this_move = m_score;
            found = true;
        }
    }
    if !found {
        panic!("Did not find a best move!!!!!! from {}", possible_moves.c);
    }
    if score_this_move[0] == 0 && score_this_move[1] == 0 && score_this_move[2] == 0
        && score_this_move[3] == 0
    {
        eprintln!("Returning a Zero move!!!");
    }
    score_this_move
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

fn sanity_check_move(
    player: usize,
    pdata: &PlayerData,
    nmove: PPos,
    board: &Board,
    tree: &MoveTree,
    node: usize,
    depth: usize,
) {
    if nmove.0 >= ::board::BWIDTH {
        panic!("Width exceeded {}");
    }
    if nmove.1 >= ::board::BHEIGHT {
        panic!("Height exceeded");
    }
    let p_cur = pdata.players[player].pos;
    let p_diff =
        (p_cur.0 as isize - nmove.0 as isize).abs() + (p_cur.1 as isize - nmove.1 as isize).abs();
    if p_diff != 1 {
        panic!(
            "Invalid move found p0({:?}) player={} move=({:?}) depth={}",
            p_cur, player, nmove, depth
        );
    }
    if board.get(nmove).0 != 0 {
        panic!("The move is already in the base board");
    }
    if tree.is_point_in(node, nmove) {
        panic!("The move is already in the move tree");
    }
    for p in pdata.iter_live() {
        let t_p = p.pos;
        if board.get(t_p).0 == 0 {
            if !tree.is_point_in(node, t_p) {
                panic!(
                    "One of the players ({}) positions {:?} is not marked as full at depth {}",
                    p.no, t_p, depth
                );
            }
        }
        if board.get(t_p).1 != p.no as u8 {
            if !tree.is_point_in(node, t_p) {
                panic!("One of the players player cells is not marked with ply no");
            }
        }
    }
}
