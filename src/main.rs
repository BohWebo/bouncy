use pancurses::Window;

extern crate pancurses;

enum VertDir {
    Up,
    Down
}

enum HorizDir {
    Left,
    Right
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: u32,
    height: u32
}

struct Game {
    frame: Frame,
    ball: Ball
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Game {
    fn new(frame_width: u32, frame_height: u32) -> Game {
        Game {
            frame: Frame {
                width: frame_width,
                height: frame_height
            },
            ball: Ball {
                x: 2,
                y: 4,
                vert_dir: VertDir::Up,
                horiz_dir: HorizDir::Left
            }
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

fn init_window_sizes(window: &Window) -> (u32, u32) {
    let (max_y, max_x) = window.get_max_yx();
    let width = (max_x - 2) as u32;
    let height = (max_y - 2) as u32;

    (width, height)
}

fn init_game(window: &Window) -> Game {
    let (width, height) = init_window_sizes(window);

    Game::new(width, height)
}


fn main () -> Result<(), String> {
    let window = pancurses::initscr();
    let mut game = init_game(&window);

    window.timeout(33);

    loop {
        window.clear(); // get rid of old content
        window.border(
            '|', // left
            '|', // right
            '-', // top
            '-', // bottom
            '+', // top left
            '+', // top right
            '+', // bottom left
            '+', // bottom right
        );
        // put the ball on the screen. Add 1 to the x and y to account
        // for the border
        window.mvaddch(game.ball.y as i32 + 1, game.ball.x as i32 + 1, 'o');
        window.mv(0, 0);
        window.refresh(); // update the screen
        // get the next bit of input
        match window.getch() {
            // exit on a q
            Some(pancurses::Input::Character('q')) => {
                pancurses::endwin();
                println!("Thanks for playing!");
                return Ok(());
            }

            // window size changed
            Some(pancurses::Input::KeyResize) => {
                game = init_game(&window);
            }

            // something else happened, just step
            _ => {
                game.step();
            }
        }
    }


    Ok(())
}