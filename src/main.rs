use ask_input::input;
use rand::Rng;

const SEPARATOR: &str = "=";
const SEPARATOR_LEN: usize = 50;
const MIN_NUMBER: i32 = 1;
const MAX_NUMBER: i32 = 100;

fn main() {
    loop {
        play_game();
        println!("Сыграем ещё? (y/n)");
        let answer = match input::<String>() {
            Ok(text) => text.trim().to_lowercase(),
            Err(_) => {
                println!("Ошибка ввода. Выход.");
                break;
            }
        };
        if answer != "y" {
            println!("Выход из игры.");
            break;
        }
    }
}

fn play_game() {
    let mut count = 0;
    let secret = rand::thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);

    println!("Угадай число от {} до {}!", MIN_NUMBER, MAX_NUMBER);
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
                        println!("Ошибка! Введите целое число от {} до {}.", MIN_NUMBER, MAX_NUMBER);
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Ошибка ввода!");
                continue;
            }
        };

        if guess < MIN_NUMBER || guess > MAX_NUMBER {
            println!("Число должно быть от {} до {}!", MIN_NUMBER, MAX_NUMBER);
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
