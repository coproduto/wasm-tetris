extern crate cfg_if;
extern crate wasm_bindgen;
// extern crate rand; // try to get this to work

use std::fmt;
use std::cmp::{min};
use wasm_bindgen::prelude::*;

use cfg_if::cfg_if;

/* feature stuff */
cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

/* random gen & debug */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = random, js_namespace = Math)]
    fn js_random() -> f64;

    #[wasm_bindgen(js_name = log, js_namespace = console)]
    fn js_log(s: &str);
}

const ROW_LEN: usize = 4;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Solid = 1,
}

type Tetromino = [Cell; 16];

pub const I: Tetromino = [Cell::Empty, Cell::Solid, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Empty, Cell::Empty];

pub const J: Tetromino = [Cell::Empty, Cell::Solid, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];

pub const L: Tetromino = [Cell::Empty, Cell::Solid, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];

pub const O: Tetromino = [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];

pub const S: Tetromino = [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Solid, Cell::Empty,
                          Cell::Solid, Cell::Solid, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];

pub const T: Tetromino = [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
                          Cell::Solid, Cell::Solid, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];

pub const Z: Tetromino = [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
                          Cell::Empty, Cell::Solid, Cell::Solid, Cell::Empty,
                          Cell::Empty, Cell::Empty, Cell::Solid, Cell::Solid,
                          Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];

const UNIVERSE_WIDTH: usize = 12;
const UNIVERSE_HEIGHT: usize = 23;
const HIDDEN_ROWS: usize = 2;

pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    fn new() -> Universe {
        Universe {
            width: UNIVERSE_WIDTH,
            height: UNIVERSE_HEIGHT,
            cells: (0..(UNIVERSE_WIDTH * UNIVERSE_HEIGHT)).map(|i| {
                if i / UNIVERSE_WIDTH < (UNIVERSE_HEIGHT - 1)
                    && i % UNIVERSE_WIDTH != 0 && (i+1) % UNIVERSE_WIDTH != 0 {
                        Cell::Empty
                    } else {
                        Cell::Solid
                    }
            }).collect(),
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width + x) as usize
    }

    fn with_matrix(&self, matrix: &CellMatrix, position: Point) -> Universe {
        let mut base = self.cells.clone();
        for i in 0..(matrix.data.len()/ROW_LEN) {
            for j in 0..ROW_LEN {
                if matrix.data[i * ROW_LEN + j] == Cell::Solid {
                    let index = self.get_index(position.x + j, position.y + i);
                    base[index] = Cell::Solid;
                }
            }
        };

        Universe {
            width: UNIVERSE_WIDTH,
            height: UNIVERSE_HEIGHT,
            cells: base,
        }
    }

    fn can_place_matrix(&self, matrix: &CellMatrix, position: Point) -> bool {
        for i in 0..(matrix.data.len()/ROW_LEN) {
            for j in 0..ROW_LEN {
                if matrix.data[i * ROW_LEN + j] == Cell::Solid
                    && self.cells[self.get_index(position.x + j, position.y + i)] == Cell::Solid {
                        return false;
                    }
            }
        }
        true
    }

    fn matrix_intersects_top(&self, matrix: &CellMatrix, position: Point) -> bool {
        for j in 0..ROW_LEN as usize {
            if matrix.data[j] == Cell::Solid && position.y < HIDDEN_ROWS {
                return true;
            }
        }
        false
    }
}

impl Universe {
    fn render_tetromino(&self, tetr: &FallingTetromino) -> String {
        let with_tetromino = self.with_matrix(&tetr.get_matrix(), tetr.position);
        with_tetromino.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let visible_cells = &self.cells[(HIDDEN_ROWS * UNIVERSE_WIDTH)..((UNIVERSE_HEIGHT - 1) * UNIVERSE_WIDTH)];
        for line in visible_cells.chunks(self.width) {
            for &cell in &line[1..(UNIVERSE_WIDTH - 1)] {
                let sym = if cell == Cell::Empty { '◻' } else { '◼' };
                write!(f, "{}", sym)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn get_random_tetromino<'a>() -> &'a Tetromino {
    let x = (js_random() * 7.0) as u8;
    match x % 7 {
        0 => &I,
        1 => &O,
        2 => &L,
        3 => &S,
        4 => &J,
        5 => &T,
        6 => &Z,
        _ => unreachable!(),
    }
}

fn get_random_rotation() -> i8 {
    (js_random() * 4.0) as i8
}

pub struct GameState<'a> {
    active_tetromino: FallingTetromino<'a>,
    universe: Universe,
}

impl<'a> GameState<'a> {
    fn render(&self) -> String {
        self.universe.render_tetromino(&self.active_tetromino)
    }
}

static mut GAME_STATE_DATA: Option<GameState> = None;
fn get_game_state() -> &'static mut GameState<'static> {
    unsafe {
        match GAME_STATE_DATA {
            None => initialize_game_state(),
            Some(ref mut state) => state,
        }
    }
}

pub fn initialize_game_state() -> &'static mut GameState<'static> {
    unsafe {
        GAME_STATE_DATA = Some(GameState {
            active_tetromino: FallingTetromino::new(),
            universe: Universe::new(),
        });
        match GAME_STATE_DATA {
            Some(ref mut state) => state,
            None => unreachable!(),
        }
    }
}

fn clear_line(game_state: &mut GameState, line_index: usize) {
    for line in (0..line_index).rev() {
        for i in (game_state.universe.width * line)..(game_state.universe.width * (line + 1)) {
            game_state.universe.cells[i + game_state.universe.width] = game_state.universe.cells[i]
        }
    }
}

fn full_line(game_state: &GameState, line_index: usize) -> bool {
    let slice = &game_state.universe.cells[((game_state.universe.width * line_index) as usize)..(game_state.universe.width * (line_index + 1))];
    slice.iter().all(|x| { *x == Cell::Solid })
}

fn drop_piece(game_state: &mut GameState) {
    // block falls
    let next_position = Point {
        x: game_state.active_tetromino.position.x,
        y: game_state.active_tetromino.position.y + 1,
    };

    let next_matrix = game_state.active_tetromino.tetromino.get_matrix();

    if game_state.universe.can_place_matrix(&next_matrix, next_position) {
        // tetromino should fall
        game_state.active_tetromino.position = next_position;
    } else {
        // tetromino should lock on to universe
        let matrix_height: usize = 4;
        game_state.universe = game_state.universe.with_matrix(&next_matrix, game_state.active_tetromino.position);
        for i in game_state.active_tetromino.position.y..min(game_state.active_tetromino.position.y + matrix_height, UNIVERSE_HEIGHT - 1) {
            if full_line(game_state, i) {
                clear_line(game_state, i);
            }
        };
        game_state.active_tetromino = FallingTetromino::new();
    }
}

#[wasm_bindgen]
pub fn render_frame() -> String {
    let game_state = get_game_state();
    game_state.render()
}

#[wasm_bindgen]
pub fn update_state() {
    let game_state = get_game_state();
    drop_piece(game_state);
}


#[wasm_bindgen]
pub fn left_input() {
    let game_state = get_game_state();

    let next_position = Point {
        x: game_state.active_tetromino.position.x - 1,
        y: game_state.active_tetromino.position.y,
    };
    let matrix = game_state.active_tetromino.tetromino.get_matrix();

    if game_state.universe.can_place_matrix(&matrix, next_position) {
        game_state.active_tetromino.position = next_position;
    }
}

#[wasm_bindgen]
pub fn right_input() {
    let game_state = get_game_state();

    let next_position = Point {
        x: game_state.active_tetromino.position.x + 1,
        y: game_state.active_tetromino.position.y,
    };
    let matrix = game_state.active_tetromino.tetromino.get_matrix();

    if game_state.universe.can_place_matrix(&matrix, next_position) {
        game_state.active_tetromino.position = next_position;
    }
}

#[wasm_bindgen]
pub fn left_rotate_input() {
    let game_state = get_game_state();
    let mut the_piece = game_state.active_tetromino;
    the_piece.rotate_left();
    let the_matrix = the_piece.get_matrix();
    if game_state.universe.can_place_matrix(&the_matrix, the_piece.position) {
        game_state.active_tetromino.rotate_left();
    }
}

#[wasm_bindgen]
pub fn right_rotate_input() {
    let game_state = get_game_state();
    let mut the_piece = game_state.active_tetromino;
    the_piece.rotate_right();
    let the_matrix = the_piece.get_matrix();
    if game_state.universe.can_place_matrix(&the_matrix, the_piece.position) {
        game_state.active_tetromino.rotate_right();
    }
}

impl<'a> FallingTetromino<'a> {
    fn new() -> FallingTetromino<'a> {
        FallingTetromino {
            tetromino: TetrominoState::new(),
            position: Point {
                x: 4, // TODO: remove magic numbers
                y: 0,
            },
        }

    }

    fn rotate_left(&mut self) {
        self.tetromino.rotate_left()
    }

    fn rotate_right(&mut self) {
        self.tetromino.rotate_right()
    }

    fn get_matrix(&self) -> CellMatrix {
        self.tetromino.get_matrix()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FallingTetromino<'a> {
    tetromino: TetrominoState<'a>,
    position: Point
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TetrominoState<'a> {
    data: &'a[Cell],
    rotation: i8,
}

pub struct CellMatrix {
    data: Vec<Cell>
}

fn transpose_matrix(m: &mut Vec<Cell>) {
    for i in 0..3 {
        for j in (i+1)..4 {
            let src  = 4 * i + j;
            let dest = 4 * j + i;
            let temp = m[src];
            m[src]  = m[dest];
            m[dest] = temp;
        }
    }
}

fn invert_rows(m: &mut Vec<Cell>) {
    for i in 0..4 {
        for j in 0..2 {
            let src  = 4 * i + j;
            let dest = 4 * i + (3 - j);
            let temp = m[src];
            m[src]  = m[dest];
            m[dest] = temp;
        }
    }
}

fn invert_cols(m: &mut Vec<Cell>) {
    for i in 0..4 {
        for j in 0..2 {
            let src  = 4 * j + i;
            let dest = 4 * (3 - j) + i;
            let temp = m[src];
            m[src] = m[dest];
            m[dest] = temp;
        }
    }
}

impl<'a> TetrominoState<'a> {
    fn new() -> TetrominoState<'a> {
        TetrominoState {
            data: get_random_tetromino(),
            rotation: get_random_rotation(),
        }
    }

    pub fn rotate_left(&mut self) {
        self.rotation += 1;
        self.rotation %= 4;
    }

    pub fn rotate_right(&mut self) {
        self.rotation -= 1;
        self.rotation %= 4;
    }

    pub fn get_matrix(self) -> CellMatrix {
        let mut data_vec: Vec<Cell> = self.data.iter().cloned().collect();

        match self.rotation {
            0 => { },
            1 | -3 => {
                // rotate right
                transpose_matrix(&mut data_vec);
                invert_cols(&mut data_vec);
            },
            2 | -2 => {
                // flip vertically
                invert_cols(&mut data_vec);
                invert_rows(&mut data_vec);
            },
            3 | -1 => {
                // rotate left
                transpose_matrix(&mut data_vec);
                invert_rows(&mut data_vec);
            },
            _ => unreachable!() // modular rotation should prevent other values
        }

        CellMatrix { data: data_vec }
    }
}
