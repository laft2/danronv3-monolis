use std::collections::VecDeque;

pub type State = [[i32; 22]; 11];

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

#[derive(Clone, Copy)]
pub struct Coord {
    pub y: i32,
    pub x: i32,
}
pub struct Game {
    pub initial_state: State,
    pub state: State,
}
impl Game {
    pub fn new(state: &State) -> Game {
        Game {
            initial_state: state.clone(),
            state: state.clone(),
        }
    }
    pub fn init(&mut self) {
        self.state = self.initial_state.clone()
    }
    pub fn next(&mut self, coord: Coord) -> bool {
        let mut target = vec![];
        let mut used = vec![vec![0; 22]; 11];
        let mut que = VecDeque::new();
        let color = self.state[coord.y as usize][coord.x as usize];
        if color == 0 {
            return true;
        }
        que.push_back(coord);
        while !que.is_empty() {
            let v = que.pop_front().unwrap();
            if used[v.y as usize][v.x as usize] == 1 {
                continue;
            }
            used[v.y as usize][v.x as usize] = 1;
            if self.state[v.y as usize][v.x as usize] != color {
                continue;
            }
            target.push(v);
            for i in 0..4 {
                let ny = v.y + DY[i];
                let nx = v.x + DX[i];
                if ny < 0 || 11 <= ny || nx < 0 || 22 <= nx {
                    continue;
                }
                if used[ny as usize][nx as usize] != 0 {
                    continue;
                }

                used[ny as usize][nx as usize] = -1;
                que.push_back(Coord { y: ny, x: nx })
            }
        }
        if target.len() <= 1 {
            return false;
        }
        used = vec![vec![0; 22]; 11];
        for ele in target {
            self.state[ele.y as usize][ele.x as usize] = 0;
            used[ele.y as usize][ele.x as usize] = 1;
            for i in 0..4 {
                let ny = ele.y + DY[i];
                let nx = ele.x + DX[i];
                if ny < 0 || 11 <= ny || nx < 0 || 22 <= nx {
                    continue;
                }
                if used[ny as usize][nx as usize] == 1 {
                    continue;
                }
                used[ny as usize][nx as usize] = 1;
                self.state[ny as usize][nx as usize] = {
                    let tmp = self.state[ny as usize][nx as usize] + 1;
                    if tmp == 5 {
                        1
                    } else if tmp == 1 {
                        0
                    } else {
                        tmp
                    }
                }
            }
        }
        true
    }
    pub fn score(&mut self, coords: &Vec<Coord>) -> i32 {
        self.init();
        let mut que = VecDeque::new();
        for ele in coords {
            que.push_back(ele);
        }
        let mut length = que.len();
        while !que.is_empty() && length != 0 {
            let v = que.pop_front().unwrap();
            let b = self.next(*v);
            if !b {
                que.push_back(v);
                length -= 1;
            } else {
                length = que.len();
            }
        }
        // for ele in coords {
        //     self.next(*ele);
        // }
        let mut score = 0;
        for ele in self.state {
            for ele in ele {
                if ele == 0 {
                    score += 1;
                }
            }
        }
        score
    }
    pub fn method(&mut self, coords: &Vec<Coord>) -> Vec<Coord> {
        self.init();
        let mut ret = vec![];
        let mut que = VecDeque::new();
        for ele in coords {
            que.push_back(ele);
        }
        let mut length = que.len();
        while !que.is_empty() && length != 0 {
            let v = que.pop_front().unwrap();
            if self.state[v.y as usize][v.x as usize] == 0 {
                continue;
            }
            let b = self.next(*v);
            if !b {
                que.push_back(v);
                length -= 1;
            } else {
                length = que.len();
                ret.push(*v);
            }
        }
        ret
    }
}
