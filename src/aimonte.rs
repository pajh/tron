//
// Simple AI find longest direction and go
//

use ai::AI;
use board::Board;
use rand::Rng;
use rand::ThreadRng;
use rand::thread_rng;
use std::ops::Index;
use support::Move;
use support::PPos;
use support::PlayerData;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct PP {
    x: u8,
    y: u8,
}
#[derive(Clone, Copy)]
struct MonteNode {
    cells: [u8; 600],
    players: [PP; 4],
    player: u8,
    plays: u16,
    wins: [u16;4],
    parent: usize,
    children: [usize; 4],
    nochildren: usize,
}

struct SMoves {
    moves: [PP; 4],
    dirs: [u8; 4],
    nodes: [usize; 4],
    len: usize,
}
impl SMoves {
    fn new() -> Self {
        let p: PP = PP { x: 255, y: 255 };
        SMoves {
            moves: [p; 4],
            dirs: [0; 4],
            nodes: [0; 4],
            len: 0,
        }
    }
    fn add(&mut self, p: PP, dir: u8) {
        self.moves[self.len] = p;
        self.dirs[self.len] = dir;
        self.len += 1;
    }

    fn addxy(&mut self, x: u8, y: u8, dir: u8) {
        self.add(PP { x, y }, dir);
    }
}

impl Index<usize> for SMoves {
    type Output = PP;

    fn index(&self, i: usize) -> &PP {
        &self.moves[i]
    }
}

const EMPTY: u8 = 16;
const C: f32 = 1.414213;

impl MonteNode {
    fn new(parent: usize, b: &Board, pdata: &PlayerData) -> Self {
        let p = PP { x: 255, y: 255 };
        let mut cells = [EMPTY; 600];
        for i in 0..600 {
            if b.cells[i] == 1 {
                cells[i] = b.play[i];
            }
        }
        let mut players = [p, p, p, p];
        for i in 0..pdata.no_players {
            if !pdata.players[i].dead {
                players[i] = PP {
                    x: pdata.players[i].pos.0 as u8,
                    y: pdata.players[i].pos.1 as u8,
                };
            }
        }
        MonteNode {
            cells: cells,
            player: 0,
            plays: 0,
            wins: [0;4],
            parent: parent,
            players: players,
            children: [0; 4],
            nochildren: 0,
        }
    }

    // Todo - does all this need to take into account each players wins - probably!
    fn monte_priority(&self, player:u8,ln_n: f32) -> f32 {

        let wins:f32 = self.wins[player as usize] as f32;
        let plays:f32 = self.plays as f32;
        //if player != 0 {
        //    wins = plays - wins;
        //}
        let p: f32 = (wins / plays ) + C * (ln_n / plays).sqrt();
        p
    }

    fn find_best_leaf(top: usize, mtree: &Vec<MonteNode>)->usize{
        let mut n = top;
        while mtree[n].nochildren > 0 {
            //eprint!("[{}]",n);
            let mut bm: usize = 0;
            let mut bv: f32 = -10.0;
            let tp = mtree[n].player;
            let ln_n = (mtree[n].plays as f32).ln();
            if mtree[n].nochildren > 1 {
                for i in 0..mtree[n].nochildren {
                    let cn = mtree[n].children[i];
                    let f = mtree[cn].monte_priority(tp,ln_n);
                    if f > bv {
                        bv = f;
                        bm = i;
                    }
                }
            }
            n = mtree[n].children[bm];
        }

        return n;
    }

    fn cpy(&self) -> Self {
        MonteNode {
            cells: self.cells,
            player: self.player,
            plays: 0,
            wins: [0;4],
            parent: self.parent,
            players: self.players,
            children: [0; 4],
            nochildren: 99,
        }
    }

    fn print_players(&self) {
        for i in 0..4 {
            if self.players[i].x == 255 {
                eprint!("[{}:DEAD]", i);
            } else {
                eprint!("[{}:({},{})]", i, self.players[i].x, self.players[i].x);
            }
        }
        eprintln!("");
    }

    fn add_child(&mut self, c: usize) {
        if c == 0 { panic!("Cannot add zero as child");}
        self.children[self.nochildren] = c;
        self.nochildren += 1;
    }

    fn bpos(p: PP) -> usize {
        p.y as usize * 30 + p.x as usize
    }

    fn bposxy(x: u8, y: u8) -> usize {
        y as usize * 30 + x as usize
    }
    // NOTE 16 is empty!
    fn get_board(&self, pp: PP) -> u8 {
        self.cells[MonteNode::bpos(pp)]
    }
    // NOTE 16 is empty!
    fn get_boardxy(&self, x: u8, y: u8) -> u8 {
        self.cells[MonteNode::bposxy(x, y)]
    }

    // Sets the players location in the player array AND marks it on the board.
    fn set_player(&mut self, player: u8, loc: PP) {
        self.players[player as usize] = loc;
        if loc.x != 255 {
            self.cells[MonteNode::bpos(loc)] = player;
        }
    }
    fn set_cplayer(&mut self, loc: PP) {
        let p = self.player;
        self.set_player(p, loc);
    }

    fn kill_player(&mut self, p: u8) {
        self.players[p as usize] = PP { x: 255, y: 255 };
        for i in 0..600 {
            if self.cells[i] == p {
                self.cells[i] = EMPTY;
            }
        }
    }

    fn next_player(&self) -> u8 {
        let mut np = self.player as usize;
        loop {
            np = (np + 1) % 4;
            if self.players[np].x != 255 {
                break;
            }
        }
        np as u8
    }

    fn alive(&self) -> usize {
        let mut alive = 0;
        for i in 0..4 {
            if self.players[i].x != 255 {
                alive += 1;
            }
        }
        alive
    }

    fn poss_moves(&self) -> SMoves {
        let pp = self.players[self.player as usize];
        if pp.x == 255 {
            panic!("Moves 4 a dead player!")
        };
        //eprint!("Checking moves for {}({},{}) [",self.player,pp.x,pp.y);
        let mut moves: SMoves = SMoves::new();
        if pp.x > 0 && self.get_boardxy(pp.x - 1, pp.y) == EMPTY {
            moves.addxy(pp.x - 1, pp.y, MSK_W);
            //eprint!("W");
        }
        if pp.x < 29 && self.get_boardxy(pp.x + 1, pp.y) == EMPTY {
            moves.addxy(pp.x + 1, pp.y, MSK_E);
            //eprint!("E");
        }
        if pp.y > 0 && self.get_boardxy(pp.x, pp.y - 1) == EMPTY {
            moves.addxy(pp.x, pp.y - 1, MSK_N);
            //eprint!("N");
        }
        if pp.y < 19 && self.get_boardxy(pp.x, pp.y + 1) == EMPTY {
            moves.addxy(pp.x, pp.y + 1, MSK_S);
            //eprint!("S");
        }
        //eprintln!("]");

        moves
    }

    fn child_with_move(&self, myndx: usize, mv: PP) -> Self {
        // Move has to be for the player active in the parent cell
        let mut child = MonteNode {
            cells: self.cells,
            player: self.next_player(),
            plays: 0,
            wins: [0;4],
            parent: myndx,
            players: self.players,
            children: [0; 4],
            nochildren: 0,
        };
        child.set_player(self.player, mv);
        child
    }

    fn gen_children(myndx: usize, mtree: &mut Vec<MonteNode>, rng: &mut ThreadRng) -> SMoves {
        loop {
            let mut moves = mtree[myndx].poss_moves();
            //eprintln!("gen_children node={}, player={}, moves={}", myndx, mtree[myndx].player, moves.len);
            //wait();

            //mtree[myndx].print_players();
            if moves.len == 0 {
                if mtree[myndx].player == 0 {
                    // If it's me then check this node aready is a 0/1 node!
                    let np = mtree[myndx].next_player() as usize;
                    MonteNode::score_to_root(mtree, myndx, np);
                    /*eprintln!(
                        "Death node for me ({}) plays:{} wins {} (IS THIS OK?)",
                        myndx,mtree[myndx].plays, mtree[myndx].wins
                    );*/
                    return moves;
                }

                if mtree[myndx].alive() == 2 {
                    // Only 2 players and the other one has died
                    MonteNode::score_to_root(mtree, myndx, 0);
                    /*eprintln!(
                        "Death node for last other player plays:{} wins {} (IS THIS OK?)",
                        mtree[myndx].plays, mtree[myndx].wins
                    );*/
                    return moves;
                }

                // One player (not me) has died but game needs to go on.
                let pdead = mtree[myndx].player;
                mtree[myndx].kill_player(pdead); // kill player
                mtree[myndx].player = mtree[myndx].next_player();
            } else {
                for i in 0..moves.len {
                    let new_child = mtree[myndx].child_with_move(myndx, moves[i]);
                    mtree.push(new_child);
                    let ncnode = mtree.len() - 1;
                    mtree[myndx].add_child(ncnode);
                    moves.nodes[i] = ncnode;
                    let r = mtree[ncnode].sim_game(rng);
                    MonteNode::score_to_root(mtree, ncnode, r);
                }
                return moves;
            }
        }
    }

    fn score_to_root(mtree: &mut Vec<MonteNode>, start_node: usize, winner: usize) {
        let mut node = start_node;
        loop {
            mtree[node].plays += 1;
            mtree[node].wins[winner] += 1;
            if node == 0 {
                break;
            }
            node = mtree[node].parent;
        }
    }

    fn sim_game(&self, rng: &mut ThreadRng) -> usize {
        // 0 = loss 1 = win

        let mut simnode = self.cpy();
        loop {
            let mut moves = simnode.poss_moves();
            loop {
                if moves.len == 0 {

                    if simnode.player == 0 {
                        return simnode.next_player() as usize;
                    }

                    if simnode.alive() == 2 {
                        return simnode.next_player() as usize;
                    }

                    // One player has died but game needs to go on.
                    let dplayer = simnode.player;
                    simnode.kill_player(dplayer); // kill player
                    simnode.player = simnode.next_player();
                    moves = simnode.poss_moves();
                } else {
                    break;
                }
            }

            let mut chose_move: usize = 0;
            if moves.len > 1 {
                chose_move = rng.gen_range(0, moves.len)
            }
            simnode.set_cplayer(moves[chose_move]);
            simnode.player = simnode.next_player();
        }
    }
}

pub struct MonteAI {
    board: Board,
    pdata: PlayerData,
    mtree: Vec<MonteNode>,
    pmap: [usize; 4], // Used to map 'games' player number to my I am always '0'
    rng: ThreadRng,
}

impl MonteAI {
    pub fn new() -> Self {
        MonteAI {
            board: Board::new(),
            pdata: PlayerData::new(0),
            mtree: Vec::new(),
            pmap: [0; 4],
            rng: thread_rng(),
        }
    }
}

const MSK_E: u8 = 1;
const MSK_W: u8 = 2;
const MSK_S: u8 = 4;
const MSK_N: u8 = 8;
const MOVES: [u8; 4] = [MSK_N, MSK_S, MSK_W, MSK_E];

impl MonteAI {
    pub fn make_move(&mut self, start_time:Instant) -> u8 {
        self.mtree.clear();

        let rootnode = MonteNode::new(0, &self.board, &self.pdata);
        self.mtree.push(rootnode);
        let moves = MonteNode::gen_children(0, &mut self.mtree, &mut self.rng);

        if moves.len == 0 {
            return 0;
        }

        if moves.len == 1 {
            return moves.dirs[0];
        }
    
        // TWEAKING THE 8 AND 92 figures
        let mut it:usize = 0;
        loop {
            if it % 8 == 0 {
                let elapsed_millis = Instant::now().duration_since(start_time).subsec_nanos() / 1_000_000;
                if elapsed_millis > 93 { break; }
            }
            let bn = MonteNode::find_best_leaf(0, &self.mtree);
            MonteNode::gen_children(bn, &mut self.mtree, &mut self.rng); // Node now needs to be a leaf node.
            it+=1;
        }
        eprintln!("Managed {} Iterations", it);

        //print_turn(&self.pdata, &self.board);
        
        let mut bscore: f32 = -1.0;
        let mut bmove: u8 = 99;
        for i in 0..moves.len {
            let mn = &self.mtree[moves.nodes[i]];
            //eprint!("[{}:{}=({}/{}/{}/{}={})]",i,moves.dirs[i],mn.wins[0],mn.wins[1],mn.wins[2],mn.wins[3],mn.plays);
            let tscore: f32 = mn.plays as f32;
            if tscore > bscore {
                bscore = tscore;
                bmove = moves.dirs[i];
            }
        }
        //eprintln!("MOVE:{}",bmove);
        //wait();
        bmove 
    }
}

pub fn print_turn(pdata: &PlayerData, b: &Board) {
    eprintln!("---------------------------------");
    for i in 0..pdata.no_players {
        if pdata.players[i].dead {
            eprint!(
                "[{}:({},{})]",
                i, pdata.players[i].pos.0, pdata.players[i].pos.1
            );
        }
    }
    eprintln!("");
    for y in 0..::board::BHEIGHT {
        let mut l = String::new();
        for x in 0..::board::BWIDTH {
            let (v, p) = b.get((x, y));
            if v == 0 {
                l.push('.');
            } else {
                if (x, y) == pdata.players[p as usize].pos {
                    l.push_str(&format!("\x1b[31m{}\x1b[0m", p));
                } else {
                    l.push_str(&format!("{}", p));
                }
            }
        }
        eprintln!("{}", l);
    }
}

impl AI for MonteAI {
    fn next_move(&mut self, p_pos: &[PPos; 4], start_time:Instant) -> Move {
        //self.handle_others(p_pos);
        AI::handle_others(&mut self.pdata, p_pos, self.pmap, &mut self.board);

        let mv = self.make_move(start_time);
        let cp = self.pdata.players[0].pos;

        let rmv = match mv {
            0 => Move::U((0, 0)),               // Dummy suicide
            MSK_N => Move::U((cp.0, cp.1 - 1)), // North
            MSK_S => Move::D((cp.0, cp.1 + 1)), // South/Down
            MSK_W => Move::L((cp.0 - 1, cp.1)), // West/Left
            MSK_E => Move::R((cp.0 + 1, cp.1)), // east/right
            _ => panic!("Invalid move"),
        };

        match self.board.is_valid_move(self.pdata.players[0].pos, rmv) {
            None => {}
            Some(new_pos) => {
                self.pdata.players[0].pos = new_pos;
                self.board.set(new_pos, 1, 0);
            }
        }

        rmv
    }

    fn start(&mut self, start_pos: &[PPos; 4], no_players: usize, you: usize, _play_rand: bool) {
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
