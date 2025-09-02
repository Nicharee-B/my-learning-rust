fn main() {
    let mut x = 5;  // mutable
    println!("The value of x is: {x}");
    x = 6;          // เปลี่ยนค่าได้
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3; // const ค่าคงที่ ต้องประกาศ type เสมอ
    println!("Three hourd in seconds is: {THREE_HOURS_IN_SECONDS}");
}