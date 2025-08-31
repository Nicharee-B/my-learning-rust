# Crate 
crate = หน่วยของโค้ดใน Rust
- ถ้าเป็น binary crate 👉 คือโปรเจกต์สามารถ build ออกมาเป็นไฟล์ executable ได้ เช่น main.rs ที่มี fn main()
- ถ้าเป็น library crate 👉 คือชุดของโค้ดหรือฟังก์ชั่น ที่เขียนมาให้โปรเจกต์อื่นเรียกใช้ จะรันตรงๆไม่ได้ เช่นในโปรเจกต์ guessing_game ที่เราจะใช้การสุ่มเลขผลลัพธ์คำตอบ ก็จะใช้เป็น library crate โค้ด/ฟังก์ชั่น crate rand สุ่มเลขที่คนอื่นเขียนไว้

✨ซึ่งวิธีเอา crate rand มาใช้เราก็ต้องเพิ่มใน depedency ที่มี cargo ช่วยจัดการ
--

# Cargo
cargo = ตัวจัดการแพ็คเกจของ Rust
มีหน้าที่ต่างๆ เช่น
1. จัดการ dependency (crate อื่นๆ ที่โปรเจกต์เราใช้)
2. สั่ง build ให้
3. สั่ง run/ test / publish ได้

## การเพิ่ม dependency
เวลาที่เราจะใช้ crate ภายนอก เช่น rand เราก็ต้องบอก cargo ก่อน โดยแก้ที่ไฟล์ Cargo.toml (อันนี้ตอนเราสร้างโปรเจกต์ด้วย cargo new นางจะจัดการสร้างให้เอง เลิศๆ)

ตรง dependencies เราเพิ่ม rand เข้าไป ถ้าเอาตามเอกสารสอน Rust Book (ดู ณ 2025/08/31) จะได้ประมาณนี้

[dependencies]
rand = "0.8.5"

🔥แต่ตอนที่เราใส่มันขึ้นกากบาทและตามด้วยเวอร์ชั่นล่าสุดเป็น 0.9.2 (อันนี้ของเราขึ้นแบบนี้ คนอื่นขึ้นแบบไหนไม่รู้) แต่เราไม่เปลี่ยนเพราะเราจะทำตามเอกสารสอนเลย อีกอย่างคือเขาบอกในเอกสารว่า 
"Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use." 
ประมาณว่า เวอร์ชัน 0.9.0 หรือสูงกว่าไม่ได้รับประกันว่าจะมี API เดียวกันกับที่ตัวอย่างที่โปรเจคใช้ไหม

# Cargo.lock
หลังจากที่เราเพิ่ม rand = "0.8.5" ใน [dependencies] แล้วเราก็ใช้คำสั่ง cargo build ที่ Terminal จากนั้น → cargo จะเลือกเวอร์ขั่น dependency ที่ตรงตามเงื่อนไขที่เราระบุใน Cargo.toml
🔹แล้ว cargo ก็จะเขียนรายละเอียดเวอร์ชันจริงๆ ที่ใช้ลงใน Cargo.lock
🔹ซึ่งต่อไปทุกครั้งที่ build Cargo จะดูจาก Cargo.lock ทำให้ได้ dependency เวอร์ชั่นเดิมเป๊ะ 
🔹ทำให้ Build reproducible คือ build ได้เหมือนเดิมทุกครั้ง ไม่ว่าจะ build ที่เครื่องเรา หรือที่เครื่องเพื่อน
🔹แต่เวอร์ชัน dependency จะไม่เปลี่ยนจนกว่าเราจะสั่งอัปเดตเอง เช่น cargo update


## สรุป Cargo.toml และ Cargo.lock
- Cargo.toml = เราเขียนเงื่อนไขเวอร์ชันของ dependency
- Cargo.lock = Cargo จะ lock เวอร์ชันจริงๆ ที่ถูกใช้ในตอน build
- เวลา push code → ควร push Cargo.lock ไปด้วย (ถ้าเป็น binary crate เช่นโปรเจกต์ guessing_game)
- ยกเว้นถ้าเป็น library crate → บางครั้งจะไม่ commit Cargo.lock เพราะอยากให้ผู้ใช้ library นั้นได้ dependency ใหม่ล่าสุดที่เข้ากันได้ในโปรเจกต์นั้นๆ