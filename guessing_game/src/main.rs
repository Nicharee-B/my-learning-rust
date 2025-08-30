//นำเข้าlibrary สำหรับการรับค่าจากผู้ใช้และส่งออก (io คือ input/output) นำเข้า io มาจากไลบรารี่มาตรฐานของ Rust>> std: 
use std::io;
fn main() {
    println!("Guess the number!");

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

    println!("You guessed: {guess}"); //{guess} คือ placeholder แทนที่ค่าด้วยค่าของตัวแปร guess ที่เราสร้าง 
}
