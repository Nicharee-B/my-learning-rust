//นำเข้า enum ที่ชื่อว่า Ordering จาก standard library ของ Rust
use std::cmp::Ordering;

//นำเข้าlibrary สำหรับการรับค่าจากผู้ใช้และส่งออก (io คือ input/output) นำเข้า io มาจากไลบรารี่มาตรฐานของ Rust>> std: 
use std::io;

/*นำเข้า rand ฟังก์ชั่นสุ่มตัวเลข ระบุ Rng เป็น trait ซึ่งจะกำหนดว่าถ้า object ไหนเป็น 
random number generator (RNG) → มันต้องมี method ที่ใช้สุ่มเลข เช่น gen, gen_range
- ถ้าไม่ use rand::Rng; → Rust จะไม่รู้ว่า method gen_range มาจากไหน → compile error*/
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //thread_rng() → สร้าง RNG ที่ปลอดภัยและผูกกับ thread ปัจจุบัน //gen_range(1..=100) → สุ่มตัวเลขในช่วง 1–100 (inclusive)


    loop {
        println!("Please input your guess.");
    
        //สร้างตัวแปรชื่อ guess เป็น String ว่าง สามารถแก้ไขเพิ่มข้อความภายหลังได้(เราสร้างอันนี้เพื่อรอเก็บค่า input จากผู้ใช้)
        let mut guess = String::new();
    
        /*ฟังก์ชั่นรับค่า input จากผู้ใช้ เพราะเราจะให้คนเล่นทายเลขใช่ป่ะ เราก็ต้องให้เขากรอกเลขมา
        stdin() คือ standard input หรือการรับข้อมูลจากผู้ใช้ผ่านคีย์บอร์ด
        .read_line(&mut guess) คือเมธอด ที่จะอ่านค่าจาก stdin() ผู้ใช้กรอกข้อมูลมา แล้วก็ส่งข้อมูลแบบแก้ไขได้(mut)นั้นไปที่ตัวแปรที่เราสร้างไว้คือ guess 
        .expect("Failed to read line") คือ ถ้าฟังก์ชั่นทำงานผิดหรือ error ก็ให้ส่งข้อความตามที่เรากำหนดกลับมา ในที่นี้คือ "Failed to read line"
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //เปลี่ยนไปใช้โค้ดข้างล่างแทน let guess: u32 = guess.trim().parse().expect("Please type a number!"); //แปลงค่า String → ตัวเลข
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); //{guess} คือ placeholder แทนที่ค่าด้วยค่าของตัวแปร guess ที่เราสร้าง 
    
        //ขั้นตอนเปรียบเทียบค่าผู้ใช้กรอก guess และค่า secret_number (ผลลัพธ์สุ่ม)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // เดาถูก หยุดลูป จบเกม
            }
        }
    }
    
}
