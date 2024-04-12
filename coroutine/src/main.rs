#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(associated_type_bounds)]
use core::num;
use std::fs::{ OpenOptions, File };
use std::io::{ Write, self, BufReader, BufRead };
use std::time::{ Duration, Instant };
use rand::Rng;

use std::ops::{ Coroutine, CoroutineState };
use std::pin::Pin;

struct WriteCoroutine {
    pub file_handle: File,
}

struct ReadCoroutine {
    lines: io::Lines<BufReader<File>>,
}

impl WriteCoroutine {
    fn new() -> Self {
        Self {
            file_handle: OpenOptions::new().create(true).append(true).open("numbers.txt").unwrap(),
        }
    }
}

impl ReadCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        Ok(Self { lines })
    }
}

impl Coroutine<()> for ReadCoroutine {
    type Yield = i32;
    type Return = ();
    fn resume(mut self: Pin<&mut Self>, arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        match self.lines.next() {
            Some(Ok(line)) => {
                if let Ok(number) = line.parse::<i32>() {
                    CoroutineState::Yielded(number)
                } else {
                    CoroutineState::Complete(())
                }
            }
            Some(Err(_)) | None => CoroutineState::Complete(()),
        }
    }
}

impl Coroutine<i32> for WriteCoroutine {
    type Return = ();
    type Yield = ();
    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.file_handle, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

fn append_number_to_file(n: i32) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open("numbers.txt")?;
    writeln!(file, "{}", n)?;
    Ok(())
}
fn main() -> io::Result<()> {
    // let mut rng = rand::thread_rng();
    // let numbers: Vec<i32> = (0..200000).map(|_| rng.gen()).collect();
    // let start = Instant::now();
    // let mut coroutine = WriteCoroutine::new();
    // for &number in &numbers {
    //     Pin::new(&mut coroutine).resume(number);
    // }
    // let duration = start.elapsed();
    // println!("Time elapsed in file operation is: {:?}", duration);
    let mut coroutine = ReadCoroutine::new("numbers.csv")?;

    loop {
        match Pin::new(&mut coroutine).resume(()) {
            CoroutineState::Yielded(number) => println!("{:?}", number),
            CoroutineState::Complete(()) => {
                break;
            }
        }
    }

    Ok(())
}
