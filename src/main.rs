fn main() {
    /*
     * mutable pada variabel, gunakan {} untuk setiap variabel
     * maka ketika mencoba mengubah nilai variabel yang sudah ada tidak ada peringatan error
     */
    let mut number = 1;
    let message1 = "hello";
    println!("message number {}: {}", number, message1);

    number = 2;
    let message2 = "world";
    println!("message number {}: {}", number, message2);

    number = 3;
    let message3: i8 = 24;
    println!("message number {1}: {0}", message3, number);

    println!("message number {}: {}", number, message3);
    println!("message number {0}: {1}", number, message3);
    println!("message number {1}: {0}", message3, number);
}
