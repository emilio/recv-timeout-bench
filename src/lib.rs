#![feature(mpsc_recv_timeout)]
#![feature(test)]
extern crate test;

use std::sync::mpsc::channel;
use std::time::Duration;
use test::Bencher;

#[bench]
fn recv_timeout_full(b: &mut Bencher) {
    let (tx, rx) = channel();
    let timeout = Duration::new(1, 0);

    b.iter(|| {
        tx.send(()).unwrap();
        rx.recv_timeout(timeout).unwrap();
    });
}

#[bench]
fn recv_full(b: &mut Bencher) {
    let (tx, rx) = channel();

    b.iter(|| {
        tx.send(()).unwrap();
        rx.recv().unwrap();
    });
}
