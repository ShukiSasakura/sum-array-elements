use clap::Parser;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    number: u64,
}

fn main() {
    const LENGTH_VEC:u64 = 10000;
    // オプションからスレッド数を取得
    let args = Args::parse();
    let thread_num:u64 = args.number;

    let sums = Arc::new(Mutex::new(vec![]));

    // 配列を用意
    let vec1:Vec<u64> = (1..LENGTH_VEC).collect();

    //時間計測を開始
    let start_time = Instant::now();

    // 与えられた数に配列を分割
    let split_num = LENGTH_VEC / thread_num;
    let mut splitted_vec_iter = vec1.chunks(split_num.try_into().unwrap());

    // 各配列をスレッドに渡し，各和を計算
    thread::scope(|s| {
        for _ in 1..=thread_num {
            let sums = Arc::clone(&sums);
            let splitted_vec = splitted_vec_iter.next();
            s.spawn(move || {
                let mut sum = 0;
                match splitted_vec {
                    None =>{},
                    Some(vec_for_calculation) => {
                        for v in vec_for_calculation {
                            sum += v;
                        }
                    }
                }
                let mut sums_element_vec = sums.lock().unwrap();
                sums_element_vec.push(sum);
            });
        }
    });

    // 各和の総和を求める
    let mut result = 0;
    let mut binding = Arc::try_unwrap(sums).unwrap();
    let sums = binding.get_mut().unwrap();
    for sum in sums {
        result += *sum;
    }

    // 計算時間を算出する
    let elapsed = start_time.elapsed();
    let elapsed_time = elapsed.as_nanos() as u64;

    // 計算時間を出力する
    println!("elapsed_time, result");
    println!("{:?}, {:?}", elapsed_time, result);

}
