pub mod core;
pub mod field;


const FIELD_SIZE: u8 = 3;


fn main() {
    let field_size = FIELD_SIZE * FIELD_SIZE;
    let mut game_field = field::GameField::new(field_size);

    core::main_loop(game_field);
}
