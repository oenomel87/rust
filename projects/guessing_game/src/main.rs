extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자 맞추기 게임!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("짐작되는 숫자를 입력해주세요: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("값을 읽을 수 없습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("당신은 {} 을/를 입력했습니다.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("정답보다 작은 숫자입니다."),
            Ordering::Greater => println!("정답보다 더 큰 숫자입니다."),
            Ordering::Equal => {
                println!("정답입니다!");
                break;
            },
        }
    }
}
