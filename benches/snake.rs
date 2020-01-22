#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use snake::snake::{Direction, Snake};

fn play_game() {
    let mut test = Snake::new(black_box(20), 10);
    for _ in 0..9 {
        test.turn(Direction::Right);
    }
    test.turn(Direction::Down);
    for _ in 0..9 {
        test.turn(Direction::Left);
    }
    test.turn(Direction::Up);
}

fn bench_snake(c: &mut Criterion) {
    c.bench_function("snake 20", |b| b.iter(|| play_game()));
}

criterion_group!(benches, bench_snake);
criterion_main!(benches);
