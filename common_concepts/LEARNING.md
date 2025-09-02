# อธิบายต่างๆ เกี่ยวกับ Common Programming Concepts ภาษา Rust
เรียน Common Programming Concepts จาก Rust Book และจะอธิบายในส่วนต่างๆที่นี่
เพื่อให้มือใหม่อย่างเราเข้าใจมากขึ้น/ขยายความได้มากขึ้น

ซึ่งในบทนี้จะมีพื้นฐาน 4 เรื่องหลักที่จำเป็นต้องเรียนรู้คือ
1. Variables and Mutability (ตัวแปรและการเปลี่ยนค่าได้)
2. Data Types (ชนิดข้อมูล)
3. Functions (ฟังก์ชัน)
4. Comments (คอมเมนต์)
5. Control Flow (โครงสร้างควบคุมการทำงาน)

## Variables and Mutability (ตัวแปรและการเปลี่ยนค่าได้)
ใน Rust การประกาศตัวแปรจะมีค่าที่แก้ไขค่าได้ในถัดๆไปกับไม่สามารถแก้ไขค่าได้ 

มี 3 แบบหลักๆ
- let = ตัวแปร (variable) ค่าเริ่มต้นจะ Immutable แก้ไขค่าไม่ได้
- let mut = ตัวแปรที่แก้ค่าได้ (mutable variable) แก้ไขค่าได้
- const = ค่าคงที่ (constant) แต่ต้องใส่ type, จะ fix ที่ compile-time
### ✨Immutable
- เวลาเราประกาศตัวแปรด้วย let ใน Rust → จะเปลี่ยนค่าไม่ได้ เพราะค่าเริ่มต้นของตัวแปรคือ immutable เปลี่ยนแปลงค่าไม่ได้
- ตัวอย่างถ้าเราเขียนแบบนี้
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; ❌ จะ error เพราะ x เปลี่ยนค่าไม่ได้น้าา เพราะ let x = 5; ประกาศไปก่อนหน้าเป็น immutable
    println!("The value of x is: {x}");
}

- error จะขึ้นว่า cannot assign twice to immutable variable `x`  แปลว่า ไม่สามารถกำหนดค่าใหม่ให้กับตัวแปรที่ immutable ได้

#### ทำไม Rust เลือกให้ immutable เป็น default
Rust ทำแบบนี้เพราะช่วยเรื่อง ความปลอดภัย (safety) และ การเขียนโปรแกรมขนาน (concurrency)
- ถ้าเราเผลอไปเปลี่ยนค่าตัวแปร โดยที่โค้ดอีกส่วนหนึ่งคิดว่าค่านี้จะไม่เปลี่ยน → อาจทำให้เกิด บัคที่หายากมาก
- การกำหนด immutable ทำให้คอมไพเลอร์ช่วยเราตรวจสอบได้ตั้งแต่ compile time (ยังไม่ต้องรันจริงก็เจอบัคแล้ว)

### ✨ถ้าอยากเปลี่ยนค่า → ใช้ mut เช่น
fn main() {
    let mut x = 5;  // mutable
    println!("The value of x is: {x}");
    x = 6;          // เปลี่ยนค่าได้
    println!("The value of x is: {x}");
}

### ✨Constants
📌 Constants ใน Rust Immutable เสมอ คือ ไม่สามารถใช้ mut ได้เลย  → ไม่ใช่แค่ immutable by default แต่ immutable ตลอดไป
- เวลาใช้ const ต้องระบุชนิดข้อมูลเสมอ const เป็น compile-time constant → ค่าจะถูกแทนลงไปในโค้ดตอนคอมไพล์เลย
    🔥คอมไพเลอร์ต้องรู้แน่ชัดว่า ค่าคงที่นี้เป็น type อะไร เพื่อที่จะคำนวณการใช้งานในโค้ด และตรวจสอบว่าค่ามีความถูกต้องตรงกับ type
- const สามารถประกาศได้ทุก scope
    🔥global scope (ใช้ได้ทั้งโปรเจกต์)
    🔥function scope (ใช้เฉพาะในฟังก์ชันนั้น)
- ชื่อตัวแปรใช้ UPPERCASE_WITH_UNDERSCORES เพื่อบอกว่าอันนี้คือค่าคงที่
#### Compile-time Constant Expression
- ค่าของ const ต้องสามารถ คำนวณได้ตั้งแต่ตอนคอมไพล์
- เช่น การคูณเลข, บวก, ลบ — compiler คิดล่วงหน้าให้ได้
- ❌ ห้ามใช้ค่าที่ต้องรอ runtime

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // ✅ OK
let runtime_value = 5;
const INVALID: u32 = runtime_value; // ❌ error

#### Constant มีผลตลอดการรันโปรแกรม
- ค่า const จะอยู่ ตลอดการทำงานของโปรแกรม
- ต่างจาก let ที่จะอยู่แค่ใน block หรือ scope ที่ประกาศ
- ใช้เก็บค่า domain-specific values เช่น Max score ในเกม, ค่าฟิสิกส์ เช่น ความเร็วแสง
ตัวอย่าง const ในเกม
const PLAYER_SPEED: f32 = 4.5;
const MAX_LEVEL: u8 = 99;

#### สรุปเมื่อไหร่ใช้ const
- เมื่อค่าคือ ค่าคงที่แน่นอน ไม่ควรเปลี่ยน เช่น ค่าเชิงฟิสิกส์, ค่าคณิตศาสตร์
- เมื่อใช้ใน type annotation (เช่น ขนาด array) ที่ compiler ต้องรู้ตั้งแต่ compile-time
- เมื่ออยากให้ compiler inline ค่าลงไปเลย เพื่อความเร็ว

### ✨Shadowing
- รอศึกษาต่อ

▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎▪︎

## Data Types (ชนิดข้อมูล)
- รอศึกษาต่อ
## Functions (ฟังก์ชัน)
- รอศึกษาต่อ
## Comments (คอมเมนต์)
- รอศึกษาต่อ
## Control Flow (โครงสร้างควบคุมการทำงาน)
- รอศึกษาต่อ

