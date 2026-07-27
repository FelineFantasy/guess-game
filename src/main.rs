use ask_input::input;
use rand::Rng;

const SEPARATOR: &str = "=";
const SEPARATOR_LEN: usize = 50;

fn main() {
    let mut count = 0;
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Угадай число от 1 до 100!");
    println!("{}", SEPARATOR.repeat(SEPARATOR_LEN));

    loop {
        print!("Введите число (или 'exit' для выхода): ");
        
        let guess = match input::<String>() {
            Ok(text) => {
                if text.trim().to_lowercase() == "exit" {
                    println!("Выход из игры.");
                    return;
                }
                match text.trim().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Ошибка! Введите целое число от 1 до 100.");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Ошибка ввода!");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("Число должно быть от 1 до 100!");
            continue;
        }

        count += 1;

        if guess == secret {
            println!("{}", SEPARATOR.repeat(SEPARATOR_LEN));
            println!("Ты угадал за {} попыток!", count);
            println!("{}", SEPARATOR.repeat(SEPARATOR_LEN));
            break;
        } else if guess < secret {
            println!("Больше {}!", guess);
        } else {
            println!("Меньше {}!", guess);
        }
        println!("{}", SEPARATOR.repeat(SEPARATOR_LEN));
    }

    println!("Нажми Enter для выхода...");
    input::<String>().unwrap();
}
