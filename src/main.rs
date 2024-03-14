use clap::Parser;
use std::{thread, f64};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::sync::mpsc;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t = 1000000)]
    length_of_vec: u64,
    #[arg(short, long, default_value_t = 400000)]
    repeat_times: u64,
    #[arg(short='n', long, default_value_t = 1)]
    thread_num: usize,
    #[arg(long, default_value_t = 1000)]
    task_size: usize,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new (thread_num: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(thread_num);
        for _ in 0..thread_num {
            let worker = Worker::new(Arc::clone(&receiver));
            workers.push(worker);
        }
        ThreadPool{
            workers,
            sender,
        }
    }

    pub fn execute <F> (&self, f: F)
        where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.threads.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    threads: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new (receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let handle = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        job.call_box();
                    },
                    Message::Terminate => {
                        break;
                    },
                }
            }
        });

        Worker{
            threads: Some(handle)
        }
    }
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>){
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

fn main() {
    // オプションから値を取得
    let args = Args::parse();
    let thread_num:usize = args.thread_num;
    let length_vec:u64 = args.length_of_vec;
    let repeat_times:u64 = args.repeat_times;
    let task_size:usize = args.task_size;

    // 配列を用意
    let calculation_vec:Vec<u64> = (1..=length_vec).collect();

    let sums = Arc::new(Mutex::new(vec![]));

    //時間計測を開始
    let start_time = Instant::now();

    // 各スレッドが一回で計算する要素の数に配列を分割
    let split_num = length_vec / <usize as TryInto<u64>>::try_into(task_size).unwrap();
    let mut splitted_vec_iter = calculation_vec.chunks(task_size);

    {
    // スレッドプールを作成
    let thread_pool = ThreadPool::new(thread_num);
    // 各配列をスレッドに渡し，各和を計算
        for _ in 0..split_num {
            let split_vec = splitted_vec_iter.next();
            let sums = Arc::clone(&sums);
            match split_vec {
                None =>{},
                Some(calc_vec) => {
                    let calc_vec = calc_vec.to_vec();
                    thread_pool.execute(move || {
                        let mut sum = 0;
                        for _ in 0..repeat_times {
                            sum = 0;
                            for v in &calc_vec {
                                sum += v;
                            }
                        }
                        let mut sums_element_vec = sums.lock().unwrap();
                        println!("sum: {:?}", sum);
                        sums_element_vec.push(sum);
                    });
                }
            }
        }
    }

    // 各和の総和を求める
    let mut result = 0;
    for sum in sums.lock().unwrap().iter() {
        result += *sum;
    }

    // 計算時間を算出する
    let elapsed = start_time.elapsed();
    let elapsed_time = elapsed.as_nanos() as f64;
    let elapsed_time = elapsed_time / 1000000000.0;

    // 計算時間を出力する
    println!("elapsed_time, result");
    println!("{:?}, {:?}", elapsed_time, result);

}
