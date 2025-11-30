pub fn main_obj_script() -> (i32, i32) {
    // this is just an example behavior for main object
    // if you run 'cargo run --bin main' you'll see the ship travelling in a square-like shape
    // so in this funciton you can write custom main object script

    static mut DIR: usize = 0; // 0 = right, 1 = down, 2 = left, 3 = up
    static mut MOVED: i32 = 0; // steps moved along current direction
    const SIDE: i32 = 50; // length of one side of square
    const SPEED: i32 = 1; // movement per frame

    // direction vectors: right, down, left, up
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let (dx, dy) = dirs[unsafe { DIR }];
    unsafe {
        MOVED += SPEED;
        if MOVED >= SIDE {
            MOVED = 0;
            DIR = (DIR + 1) % 4; // rotate to next direction
        }
    }

    (dx * SPEED, dy * SPEED)

    // this is the default behavior -- the ship is not moving unless some key is pressed
    // (0, 0)
}
