use clap::Parser;
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    number: u32,
}

fn main() {
    const LENGTH_VEC:usize = 10000;
    // オプションからスレッド数を取得
    let args = Args::parse();
    let thread_num = args.number;

    let mut handles = vec![];
    let mut sums = vec![];

    // 配列を用意
    let mut vec1: Vec<u64> = Vec::with_capacity(LENGTH_VEC);
    for i in 1..=LENGTH_VEC {
        vec1.push(i.try_into().unwrap());
    }

    // 与えられた数に配列を分割
    let split_num =<usize as TryInto<u32>>::try_into(LENGTH_VEC).unwrap() / thread_num;
    let mut splitted_vec_iter = vec1.chunks(split_num.try_into().unwrap());
    // 各配列をスレッドに渡し，各和を計算
    for i in 1..=thread_num {
        let mut splitted_vec = splitted_vec_iter.next();
        let handle = thread::spawn(move || {
            let mut sum = 0;
            match splitted_vec {
                None =>{},
                Some(vec_for_calculation) => {
                    for j in 0..vec_for_calculation.len() {
                        sum = sum + vec_for_calculation[j];
                    }
                }
            }
            sums.push(sum);
        });
        handles.push(handle);

    }
    // 計算終了を待つ
    for handle in handles {
        handle.join().unwrap();
    }

    // 各和の総和を求める
    let mut result = 0;
    for sum in sums {
        result = result + sum;
    }
    println!("{:?}", result);
}
