fn main() {
    /*
     * signed integer
     * i8 = −(2^7) hingga (2^7)-1
     * i16 = −(2^15) hingga (2^15)-1
     * i32 = −(2^31) hingga (2^31)-1
     * i64 = −(2^63) hingga (2^63)-1
     * i128 = −(2^127) hingga (2^127)-1
     */
    let numerik1 = 24;
    // let numerik2: i8 = 128; /* akan muncul error, karena lebih dari 127 */
    let numerik3: i64 = 12;
    let min_i128 = i128::MIN;
    let max_i128 = i128::MAX;
    println!("{} | {}", min_i128, max_i128);

    println!("{} | {}", numerik1, numerik3);
}
