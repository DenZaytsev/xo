use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Cell{
    X,
    O,
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub struct GameField{
    pub  field: Vec<Cell>, 
    pub size : u8,
}

impl GameField {
    pub fn new(size: u8) -> Self{
        let empty_field = vec![Cell::Empty; size as usize];
        return Self { field: empty_field, size: size}
    }

    pub fn make_move(&mut self, ceil_num: u8, value: Cell){
        self.field[ceil_num as usize] = value;
    }

    pub fn check_win_condition(&self) -> bool{
        true
    }

    fn check_diagonals(&self) -> bool {
        true
    }
}


impl fmt::Display for GameField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.field;
        let side_size = (self.size as f64).sqrt() as usize;

        for (count, cell) in vec.iter().enumerate() {

            if count%side_size == 0 {write!(f, "\n\t")?;}
            write!(f, " [{}|{}] ", cell, count)?;
        }
        return write!(f, "\n")
    }
}