extern crate rand;

mod board {
use std;
use support::MAXPLAYERS;
use support::Move;
use support::PPos;
use support::PlayerData;

pub const BWIDTH: usize = 30;
pub const BHEIGHT: usize = 20; // 0,0 is top left

pub struct Board {
    pub cells: [u16; 600],
    pub play: [u8; 600],
}

impl Board {

    pub fn cpy(&self) -> Self {
        let ncells = self.cells;
        let nplay = self.play;
        Board { cells:ncells, play:nplay }
    }

    pub fn new() -> Self {
        Board {
            cells: [0; 600],
            play: [MAXPLAYERS as u8; 600],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..600 {
            self.cells[i] = 0;
            self.play[i]= MAXPLAYERS as u8;
        }
    }

    pub fn pos(p: PPos) -> usize {
        p.1 * BWIDTH + p.0
    }

    pub fn set(&mut self, p: PPos, v: u16, player: u8) {
        let ps = Board::pos(p);
        self.cells[ps] = v;
        self.play[ps] = player;
    }

    pub fn get(&self, p: PPos) -> (u16, u8) {
        let ps = Board::pos(p);
        (self.cells[ps], self.play[ps])
    }

    pub fn clear_player_from_board(&mut self, p: u8) {
        for i in 0..600 {
            if self.play[i] == p {
                self.play[i] = MAXPLAYERS as u8;
                self.cells[i] = 0;
            }
        }
    }

    pub fn is_valid_move(&self, pos: PPos, m: Move) -> Option<PPos> {
        match m {
            Move::U(_) => {
                return if pos.1 > 0 && self.get((pos.0, pos.1 - 1)).0 == 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }

            Move::D(_) => {
                return if pos.1 < BHEIGHT - 1 && self.get((pos.0, pos.1 + 1)).0 == 0 {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }

            Move::L(_) => {
                return if pos.0 > 0 && self.get((pos.0 - 1, pos.1)).0 == 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }

            Move::R(_) => {
                return if pos.0 < BWIDTH - 1 && self.get((pos.0 + 1, pos.1)).0 == 0 {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
        }
    }

    pub fn is_valid_ret_nxt(&self, m: Move) -> Option<Move> {
        match m {
            Move::U(p) => {
                return if p.1 > 0 && self.get((p.0, p.1 - 1)).0 == 0 {
                    Some(Move::U((p.0, p.1 - 1)))
                } else {
                    None
                }
            }

            Move::D(p) => {
                return if p.1 < BHEIGHT - 1 && self.get((p.0, p.1 + 1)).0 == 0 {
                    Some(Move::D((p.0, p.1 + 1)))
                } else {
                    None
                }
            }

            Move::L(p) => {
                return if p.0 > 0 && self.get((p.0 - 1, p.1)).0 == 0 {
                    Some(Move::L((p.0 - 1, p.1)))
                } else {
                    None
                }
            }

            Move::R(p) => {
                return if p.0 < BWIDTH - 1 && self.get((p.0 + 1, p.1)).0 == 0 {
                    Some(Move::R((p.0 + 1, p.1)))
                } else {
                    None
                }
            }
        }
    }
    pub fn to_string(&self, pmap: [usize; 4]) -> String {
        let mut s = "".to_string();
        for y in 0..BHEIGHT {
            for x in 0..BWIDTH {
                let (v, p) = self.get((x, y));
                if v == 0 {
                    s.push_str("0 ");
                } else {
                    s.push_str(&format!("{} ", pmap[p as usize]));
                }
            }
            s.push_str("\n");
        }
        s
    }
}

#[derive(Debug)]
pub struct AvailMoves {
    pub moves: [Move; 4],
    pub score: [i16; 4],
    pub c: usize,
}

impl AvailMoves {
    pub fn new() -> Self {
        let t_m = Move::U((0, 0));
        AvailMoves {
            moves: [t_m, t_m, t_m, t_m],
            score: [0; 4],
            c: 0,
        }
    }

    pub fn print(&self) {
        for i in 0..self.c {
            eprint!("{}", self.moves[i].to_debug());
        }
        eprintln!(".");
    }

    pub fn iter(&self) -> std::slice::Iter<Move> {
        self.moves[0..self.c].iter()
    }

    pub fn add(&mut self, mov: Move) {
        self.moves[self.c] = mov;
        self.c += 1;
    }

    pub fn calc_board(b: &Board, p: PPos) -> Self {
  
        let all_options = AvailMoves::calc(p);
        if all_options.len() == 0 {
            return all_options;
        }

        let mut checked_options = AvailMoves::new();
        for mv in all_options.iter() {
            if b.get(mv.get_point()).0 == 0 {
                checked_options.add(*mv);
            }
        }
        checked_options
    }

    pub fn is_empty(b: &Board, tree: &MoveTree, node: usize, p: PPos, pdata: &PlayerData) -> bool {
        let v = b.get(p);
        if v.0 == 0 {
            return !tree.is_point_in_notdead(node, p, &pdata);
        } // empty but might have a tree entry

        let pyr = v.1 as usize; // player whose trail blocks square - might be dead

        if pdata.is_dead(pyr) == false {
            // player blocking is still alive
            return false;
        }
        !tree.is_point_in_notdead(node, p, &pdata)
    }

    // Takes into account if square is 'filled' by a dead player
    pub fn calc_board_tree_dead(
        b: &Board,
        tree: &MoveTree,
        node: usize,
        p: PPos,
        pdata: &PlayerData,
    ) -> Self {

        let all_options = AvailMoves::calc(p);

        let mut checked_options = AvailMoves::new();
        for mv in all_options.iter() {
            let t_p = mv.get_point();
            //if is_empty(b, tree, node, p, pdata)
            //         if ((b.get(t_p).0 == 0) || (b.get(t_p).0 != 0 && pdata.is_dead(b.get(t_p).1 as usize)))
            //             && !tree.is_point_in_notdead(node, t_p, &pdata)
            if AvailMoves::is_empty(b, tree, node, t_p, pdata) {
                checked_options.add(*mv);
            }
        }
        checked_options
    }
    // Just gets valid moves (ie checks the edges only - no checking of board contents)
    pub fn calc(p: PPos) -> Self {
        let mut av = AvailMoves::new();
        if p.0 > 0 {
            av.add(Move::L((p.0 - 1, p.1)));
        }

        if p.0 < BWIDTH - 1 {
            av.add(Move::R((p.0 + 1, p.1)));
        }

        if p.1 > 0 {
            av.add(Move::U((p.0, p.1 - 1)));
        }

        if p.1 < BHEIGHT - 1 {
            av.add(Move::D((p.0, p.1 + 1)));
        }
        av
    }

    pub fn tree_check(self, tree: &MoveTree, node: usize) -> Self {
        let mut nv = AvailMoves::new();

        // let tm = Move::U( (1,1));

        for i in 0..self.c {
            let p = self.moves[i].get_point();
            if !tree.is_point_in(node, p) {
                nv.add(self.moves[i]);
            }
        }

        nv
    }

    pub fn contains(&self, m: &Move) -> bool {
        if self.c == 0 {
            return false;
        };
        for i in 0..self.c {
            if self.moves[i] == *m {
                return true;
            };
        }
        false
    }

    pub fn getxy(&self, index: usize) -> PPos {
        if index >= self.c {
            panic!("Invalid position asked for");
        }
        self.moves[index].get_point()
    }

    pub fn len(&self) -> usize {
        return self.c;
    }
}

struct MoveTreeData {
    pos: PPos,
    prev: usize,
    play: u8,
}
pub struct MoveTree {
    //pub next_free: usize,   //TODO: remove these pubs
    tree: Vec<MoveTreeData>,
}

impl MoveTree {
    pub fn new() -> Self {
        eprintln!("MoveTree new");
        let mut tree = MoveTree { tree: Vec::new() };
        tree.clear();
        tree
    }

    pub fn get(&self, node: usize) -> (PPos, u8, usize) {
        // Position, player, prev
        let n_t = &self.tree[node];
        ((n_t.pos), n_t.play, n_t.prev)
    }

    pub fn to_str(&self, node: usize) -> String {
        let mut text: String = "".to_string();
        let mut n = node;
        while n != 0 {
            let t_n = &self.tree[n];
            let f = format!("->({},{})", t_n.pos.0, t_n.pos.1);
            text.push_str(&f);
            n = t_n.prev;
        }
        text
    }

    pub fn clear(&mut self) {
        self.tree.clear();
        self.tree.push(MoveTreeData {
            pos: (999, 999),
            prev: 999999,
            play: 99,
        }); // Dummy Zero node
    }

    pub fn add(&mut self, point: PPos, parent: usize, player: u8) -> usize {
        let new_data = MoveTreeData {
            pos: point,
            prev: parent,
            play: player,
        };
        self.tree.push(new_data);

        self.tree.len() - 1
    }

    pub fn is_point_in(&self, node: usize, point: PPos) -> bool {
        let mut c = node;

        while c != 0 {
            let t_node = &self.tree[c];
            if t_node.pos == point {
                return true;
            }
            c = t_node.prev;
        }
        false
    }

    pub fn is_point_in_notdead(&self, node: usize, point: PPos, pdata: &PlayerData) -> bool {
        let mut c = node;
        //println!("in is_point_in_notdead for {:?}", point);
        while c != 0 {
            let t_node = &self.tree[c];
            if t_node.pos == point {
                return !pdata.players[t_node.play as usize].dead;
            }
            c = t_node.prev;
        }
        false
    }
}
} 


mod support {
use std::cell::Cell;
use std::fs::OpenOptions;
use std::io::Write;

pub const MAXPLAYERS: usize = 4;
pub type PPos = (usize, usize);
pub type PScore = [i16; MAXPLAYERS];
pub const ZEROSCORE: PScore = [0; MAXPLAYERS];

pub fn ppos_to_str(p: PPos) -> String {
    format!("({},{})", p.0, p.1)
}

#[derive(Copy, Clone)]
pub struct Fill {
    pub depth: u16,
    pub pos: PPos,
    pub player: u8,
}

impl Fill {
    pub fn new()->Self {
        Fill { depth:0, pos: (0,0), player: 0}
    }
}

#[derive(Copy, Clone)]
pub struct Fill2 {
    pub depth: u16,
    pub pos: PPos,
    pub player: u8,
    pub orientation:u8,
}

impl Fill2 {
    pub fn new()->Self {
        Fill2 { depth:0, pos: (0,0), player: 0, orientation:0}
    }
}

#[derive(PartialEq, Clone, Copy,Debug)]
pub enum Move {
    L(PPos),
    R(PPos),
    U(PPos),
    D(PPos),
}

impl Move {
    pub fn get_point(&self) -> PPos {
        match *self {
            Move::L(p) => p,
            Move::R(p) => p,
            Move::U(p) => p,
            Move::D(p) => p,
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Move::L(_) => "LEFT".to_string(),
            Move::R(_) => "RIGHT".to_string(),
            Move::U(_) => "UP".to_string(),
            Move::D(_) => "DOWN".to_string(),
        }
    }

    pub fn to_debug(&self) -> String {
        match *self {
            Move::L(a) => "LEFT".to_string() + &ppos_to_str(a),
            Move::R(a) => "RIGHT".to_string() + &ppos_to_str(a),
            Move::U(a) => "UP".to_string() + &ppos_to_str(a),
            Move::D(a) => "DOWN".to_string() + &ppos_to_str(a),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    pub no: usize,
    pub pos: PPos,
    pub dead: bool,
}

#[derive(Copy, Clone)]
pub struct PlayerData {
    pub no_players: usize,   
    pub players: [Player; MAXPLAYERS],
}

impl PlayerData {
    pub fn new(no_players: usize) -> Self {
        
        let dummy = Player {
            no: 0,
            pos: (0, 0),
            dead: false,
        };
        let mut pd = PlayerData {
            no_players: no_players,           
            players: [dummy; MAXPLAYERS],
        };
        for i in 0..MAXPLAYERS {
            pd.players[i].no = i;
        }
        
        pd
    }
    
    pub fn iter_live(&self) -> ::std::iter::Filter<::std::slice::Iter<Player>, fn(&&Player) -> bool> {
        //    fn iter(&self) -> std::slice::Iter<Player> {
        let t_slice = &self.players[0..self.no_players];
        t_slice.iter().filter(|s| s.dead == false)
    }

    pub fn next_live_player(&self, player: usize) -> usize {
        let mut next_player = player;
        loop {
            next_player = (next_player + 1) % self.no_players;
            if self.players[next_player].dead == false {
                return next_player;
            }
        }
    }

    pub fn me(&self) -> Player {
        self.players[0]
    }

    pub fn live_count(&self) -> usize {
        let mut count: usize = 0;
        for i in 0..self.no_players {
            if self.players[i].dead == false {
                count += 1;
            }
        }
        count
    }

    pub fn is_dead(&self, pno: usize) -> bool {
        self.players[pno].dead
    }

    /*    fn getpos(&self) -> &[PPos] {
        &self.pos[0..self.no_players]
    }*/
}

pub fn append_str_to_file(filename: &str, data:&str) {

    let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .unwrap();
    file.write_all(data.as_bytes()).expect("Unable to write");;

}
}


mod aimonte {
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
}


use aimonte::MonteAI;

use std::io;

mod ai {
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
}


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