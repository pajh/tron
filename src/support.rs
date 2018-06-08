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
