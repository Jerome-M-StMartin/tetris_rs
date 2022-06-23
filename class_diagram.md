:::mermaid

classDiagram

%% Classes
class Tetromino {
    [Option~usize>; 4]: occupied_indices
    RotationVariant: rotation

    +void draw(u8, u8)
    +void drop(&grid)
    +void try_rotate(&grid)
    +void try_move(&grid, u8, u8)

    -bool can_move(u8, u8)
}

class RotationEnum {
    Variant::0,
    Variant::90,
    Variant::180,
    Variant::270,
}

class Model {
    Rc~RefCell<[char; 400]>>: grid
    [Tetromino; 8]: tetrominoe_queue
    Option~Tetromino>: stored_tetromino
    Tetromino: curr_tetromino
    u8, u8: curr_tetro_pos
    Instant: timer
    Duration: timeout_interval
    u8: num_tetros_placed

    +void tick(Option~InputEvent>)
    -void store_swap_tetro()
    -Tetromino queue_pop()
    -Tetromino gen_random_tetro()
    -usize cull_full_rows()
    -void increase_difficulty()
    -void game_over()
    -void pause()    
    -usize: xy_to_idx(u8, u8)
    -u8, u8: idx_to_xy(usize)
}

class View {
    Rc~RefCell<[char; 400]>>: grid
    crossterm: terminal

    +void draw()
}

class Controller {
    UserInput: STATELESS, PURE FNs ONLY

    +InputEvent: poll_read_input()
    -InputEvent: translate_event(crossterm_event)
}

class MainLoop {
    +void tick()
}

Model --* Tetromino
Tetromino --o RotationEnum
Controller --> MainLoop
MainLoop --> View
MainLoop --> Model

:::