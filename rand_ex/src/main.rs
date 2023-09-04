use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Standard};
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{distributions::Uniform, Rng};
use rand_distr::Normal;
use std::error;

// rand book: https://rust-random.github.io/book/
fn main() -> Result<(), Box<dyn error::Error>> {
    // 1. 生成随机数值
    println!("----------------  生成随机数值  --------------------");
    let mut rng = rand::thread_rng();
    println!("Random u8: {}", rng.gen::<u8>());
    println!("Random u16: {}", rng.gen::<u16>());
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>()); // 浮点数限制在 [0, 1)

    // 2. 生成随机范围内的值
    println!("----------------  生成随机范围内的值  --------------------");
    // 随机范围值
    println!("----->  随机范围值 ");
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..=10)); // 限制在 [0, 1]
    println!("Float: {}", rng.gen_range(0.0..10.0)); // 限制在 [0.0, 10.0)

    // 可以均匀的在范围内生成值, 重复生成值时, 会更快
    println!("----->  Uniform 均匀的在范围内生成值 ");
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    println!("----------------  rand_distr 自定义其它类型的随机值分布  --------------------");
    // rand_distr crate 提供其它类型的随机值分布, 例如均值和标准偏差.
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);

    println!("----------------  自定义类型生成随机值 tuple, struct  --------------------");
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

    println!("----------------  通过一组 ASCII 生成随机密码  --------------------");
    println!(
        "rand password use Alphanumeric: {}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>()
    );
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    println!(
        "rand password use custom characters : {:?}",
        (0..30)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect::<String>()
    );

    println!("----------------  从 &[T] 随机选一个  --------------------");
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    let letter = letters.iter().choose(&mut rng);
    println!("{:?}", letter);

    println!("----------------  shuffle 洗牌算法  --------------------");
    let mut letters = letters;
    letters.shuffle(&mut rng);
    println!("{:?}", letters);

    Ok(())
}

// 自定义类型生成随机值
#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}
