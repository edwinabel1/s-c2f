use std::io::{self, Write};

fn main() {
    println!("摄氏温度（°C）？如果是华氏温度直接回车");

    let mut cel = String::new();
    io::stdin().read_line(&mut cel).expect("读取失败");
    match cel.as_str() {
        "\r\n" => {
            print!("华氏温度（°F）？");
            io::stdout().flush().expect("err_flush()");
            let mut fahr = String::new();
            io::stdin().read_line(&mut fahr).expect("读取失败");
            let fahr = fahr.trim().parse().unwrap();
            println!("摄氏温度：{}°C", fahr2cel(fahr))
        }
        _ => {
            let cel: f64 = cel.trim().parse().unwrap();
            println!("华氏温度：{}°F", cel2fahr(cel));
        }
    }
}

fn cel2fahr(cel: f64) -> f64 {
    cel * 1.8 + 32.0
}

fn fahr2cel(fahr: f64) -> f64 {
    (fahr - 32.0) / 1.8
}
