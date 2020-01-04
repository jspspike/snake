#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use snake::snake::{Snake, Direction};

fn bench_snake(c: &mut Criterion) {
    let mut test = Snake::new(black_box(20), 10);
    c.bench_function("snake 20", |b| b.iter(|| test.turn(Direction::Center))); 
}

criterion_group!(benches, bench_snake);
criterion_main!(benches);
