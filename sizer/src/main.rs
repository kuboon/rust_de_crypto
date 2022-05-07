use core::ascii;
use std::io;

fn take(v: Vec<i32>) -> Vec<i32> {
    return v;
}

fn inp() {}
use std::io::*;

fn コンソール入力() -> String {
    let mut バッファ = String::new();
    let c: char = 'c';
    let str: &str;
    stdin().read_line(&mut バッファ).unwrap();

    //&str=&バッファ.to_string(); //.trim().to_string();
    //c=&str;

    let num: i32 = c as i32 - 48;
    println!("{}", num); // 5

    return バッファ;
}

fn str2S(str: &str) -> String {
    let s1: String = String::from(str);
    println!("{}", s1);

    return s1;
}

fn S2c(a: String) -> Vec<char> {
    let cs: Vec<char> = a.chars().collect();
    //let ss:[char;32];
    let i: i32;
    let b: char;
    b = cs[0];
    for i in 0..a.len() - 1 {
        print!("{}", cs[i]);
    }
    return cs;
}

fn c2S(c: Vec<char>) -> String {
    //let c: char = 'a';
    let cs: String = c.iter().collect();
    println!("{}", &cs); // → a
    return cs;
}

fn main() {
    //inp();
    let gf: [i32; 256] = [
        0, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180,
        117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212,
        181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111,
        222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253,
        231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67,
        134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199,
        147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158,
        33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213,
        183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255,
        227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100,
        200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239,
        195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235,
        203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142,
    ];
    let fg: [i32; 256] = [
        0, 1, 2, 26, 3, 51, 27, 199, 4, 224, 52, 239, 28, 105, 200, 76, 5, 101, 225, 15, 53, 142,
        240, 130, 29, 194, 106, 249, 201, 9, 77, 114, 6, 139, 102, 48, 226, 37, 16, 34, 54, 148,
        143, 219, 241, 19, 131, 70, 30, 182, 195, 126, 107, 40, 250, 186, 202, 155, 10, 121, 78,
        229, 115, 167, 7, 192, 140, 99, 103, 222, 49, 254, 227, 153, 38, 180, 17, 146, 35, 137, 55,
        209, 149, 207, 144, 151, 220, 190, 242, 211, 20, 93, 132, 57, 71, 65, 31, 67, 183, 164,
        196, 73, 127, 111, 108, 59, 41, 85, 251, 134, 187, 62, 203, 95, 156, 160, 11, 22, 122, 44,
        79, 213, 230, 173, 116, 244, 168, 88, 8, 113, 193, 248, 141, 129, 100, 14, 104, 75, 223,
        238, 50, 198, 255, 25, 228, 166, 154, 120, 39, 185, 181, 125, 18, 69, 147, 218, 36, 33,
        138, 47, 56, 64, 210, 92, 150, 189, 208, 206, 145, 136, 152, 179, 221, 253, 191, 98, 243,
        87, 212, 172, 21, 43, 94, 159, 133, 61, 58, 84, 72, 110, 66, 163, 32, 46, 68, 217, 184,
        124, 165, 119, 197, 24, 74, 237, 128, 13, 112, 247, 109, 162, 60, 83, 42, 158, 86, 171,
        252, 97, 135, 178, 188, 205, 63, 91, 204, 90, 96, 177, 157, 170, 161, 82, 12, 246, 23, 236,
        123, 118, 45, 216, 80, 175, 214, 234, 231, 232, 174, 233, 117, 215, 245, 235, 169, 81, 89,
        176,
    ];

    let mut str1: String = String::from("5");
    let num: i32 = str1.parse().unwrap();
    println!("{}", num); // 5
    let mut str: &str;
    let c: Vec<char> = "ajin".chars().collect();
    let mut x:Vec<char>;
    let num: i32 = c[0] as i32 - 48;
    println!("{}", num); // 5
    str = "fuck!";
    str1 = str2S(str);
    let ss3: &str = &str1;
    println!("{}", ss3);
    //exit();
    //use scanf::sscanf;

    let input: &str = "Candy: 2.75";
    let s2: String = String::from(input);
    //let s3:&str=&s2;
    let sa: Vec<char>;
    sa = S2c(s2);

    let i: i32;
    println!("");
    for i in 0..10 {
        println!("char? {}", sa[i]);
    }
    //let mut s3:&str=&s2;
    //println!("{}",s3);
    let mut product: String = String::new();
    let mut wo: String = String::new();
    let r: Vec<char>;
    std::io::stdin().read_line(&mut wo).ok();
    let answer = wo.trim().to_string();
    r = S2c(answer);
    let i: i32;
    for i in 0..r.len() {
        println!("ans={}", r[i]);
    }

    str1 = c2S(c);
    println!("{}", str1); // abcdef

    let _i: i32;
    let _j: i32;
    let a = 101;
    const size: i32 = 256;

    let mut c: [i32; 257]=[0;257];
    let mut buf: [i32; 257]=[0;257];
    let m: [i32; 257];

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("You guess: {}", guess);
    //let parsed = sscanf::scanf!(&mut m, "Hello {}{}!", str, usize);
    println!("何か入力を");
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).ok();
    println!("{}", data);
    let v = &data[0..15];
    let byte:&[u8]=v.as_bytes();
    let i: i32;
    for i in 0..byte.len() {
        println!("v[{}]={}", i, byte[i] as char);
    }
    
    let mut j;
    let mut ii:i32;
    //x=S2c(data);
    j = byte.len();
    for ii in 0..j {
        c[ii] = (gf[(((byte[ii]) as usize) + (a )) % 256]);
    }

    for i in 0..(j as usize) {
        print!("{},", c[i] as usize);
    }
    println!("");

    print!("{}\n", byte[(j) - 1]);

    for i in 0..(j) {
        buf[i] = (fg[(c[i] as usize)]) - (a as i32);
    }
    let mut w:[u8;257]=[0;257];
    for i in 0..j{
        w[i]=(buf[i]%256) as u8;
    }
    for i in 0..16{
    print!("{} ", w[i] as char);
    }
    print!("\n");
}