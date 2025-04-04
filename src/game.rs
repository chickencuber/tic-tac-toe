use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Turn {
    X = 1,
    O = -1,
    None = 0,
    Multi = 2,
}


impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Turn::X => "X",
            Turn::O => "O",
            Turn::None => " ",
            Turn::Multi => "Multiplayer",
        })
    }
}

fn check(board: [Turn; 9], a: usize, b: usize, c: usize) -> bool{
    let a = board.get(a).unwrap();
    let b = board.get(b).unwrap();
    let c = board.get(c).unwrap();
    return a == b && a == c && a != &Turn::None;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EndType {
    None,
    Win,
    Draw,
}

pub fn is_won(board: [Turn; 9]) -> EndType {
    let checkl = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),

        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),

        (0, 4, 8),
        (6, 4, 2),
    ];
    for (a, b, c) in checkl {
        if check(board, a, b, c) {
            return EndType::Win;
        }
    }
    for t in board {
       if t == Turn::None {
           return EndType::None;
       } 
    }
    return EndType::Draw;
}


#[derive(Debug)]
struct Move {
    index: usize,
    board: Boards,
}

#[derive(Debug)]
struct Boards {
    score: i8,
    end: bool,
    board: [Turn; 9],
    moves: Vec<Move>,
}

fn get_moves(board: [Turn; 9]) -> Vec<usize> {
    let mut vec = Vec::new();
    for (i, t) in board.iter().enumerate() {
        if *t == Turn::None {
            vec.push(i)
        }
    }
    return vec;
}

pub fn new_turn(turn: Turn) -> Turn {
    return match turn {
        Turn::X => Turn::O,
        Turn::O => Turn::X,
        _ => Turn::None,
    }
}

fn get_boards(turn: Turn, board: [Turn; 9]) -> Boards {
    let w = is_won(board);
    if w != EndType::None {
        if w == EndType::Draw {
            return Boards {
                score: 0,
                end: true,
                moves: Vec::new(),
                board: board.clone(),
            }
        } else {
            return Boards {
                score: new_turn(turn) as i8,
                end: true,
                moves: Vec::new(),
                board: board.clone(),
            }
        }
    }
    let moves = get_moves(board);
    let mut vec = Vec::new();
    let mut s = if turn == Turn::X {-1} else {1};
    for i in moves {
        let mut b = board.clone();
        b[i] = turn;
        let mut boards = get_boards(new_turn(turn), b);
        if turn == Turn::X {
            if boards.score > s {
                s = boards.score;
            }
        } else{
             if boards.score < s {
                s = boards.score;
            }           
        }
        if boards.end {
            boards.score *= 2;
        }
        vec.push(
            Move{
                board: boards,
                index: i,
            });
    }
    return Boards {
        score: s,
        end: false,
        moves: vec,
        board: board.clone()
    }
}

pub fn board_to_string(board: [Turn; 9]) -> String {
    let mut s = String::new();
    for t in board {
        s.push_str(t.to_string().as_str());
    } 
    return s;
}


fn add_boards(hash: &mut HashMap<String, Vec<usize>>, boards: Boards) {
    let key = board_to_string(boards.board);
    if !hash.contains_key(&key) {
        let mut moves: Vec<(i8, usize)> = boards.moves.iter().map(|x| (x.board.score, x.index)).collect();
        moves.sort_by(|a, b| b.0.cmp(&a.0));
        let new_moves: Vec<usize> = moves.iter().map(|x| x.1).collect();
        hash.insert(key, new_moves);
    }
    for b in boards.moves {
       add_boards(hash, b.board); 
    }
}

pub fn calc_ai() -> HashMap<String, Vec<usize>> {
    let boards = get_boards(Turn::X, [Turn::None; 9]);
    let mut hash = HashMap::new();
    add_boards(&mut hash, boards);
    return hash;  
}

