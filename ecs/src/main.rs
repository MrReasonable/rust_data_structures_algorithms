use ecs::{
    data::{Dir, Pos, Strength},
    gen::GenManager,
    store::{EcsStore, VecStore},
    systems,
};
use std::io::Write;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let (ch_s, ch_r) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for k in stdin.keys() {
            ch_s.send(k).ok();
        }
    });

    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut gen = GenManager::new();
    let mut strengths = VecStore::new();
    let mut dirs = VecStore::new();
    let mut poss = VecStore::new();

    let mut pass = 0;

    loop {
        let g = gen.next();
        strengths.add(g, Strength { s: 1, h: 5 });
        dirs.add(g, Dir { vx: 0, vy: 0 });
        poss.add(
            g,
            Pos {
                x: (rand::random::<i32>() % w),
                y: (rand::random::<i32>() % h),
            },
        );
        systems::dir_sys(&mut dirs, &poss);
        systems::move_sys(&dirs, &mut poss);
        systems::collision_sys(&poss, &mut strengths);
        systems::death_sys(&mut gen, &mut strengths, &mut poss, &mut dirs);
        systems::render_sys(&mut screen, &poss, &strengths);
        write!(&mut screen, "{}Pass={}", termion::cursor::Goto(1, 1), pass).ok();
        pass += 1;

        screen.flush().ok();

        while let Ok(Ok(k)) = ch_r.try_recv() {
            match k {
                Key::Char('q') => return,
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(150))
    }
}
