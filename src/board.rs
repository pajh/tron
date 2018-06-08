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
