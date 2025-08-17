fn main() {
    /*
     * variabel tanpa didefinisikan tipenya
     */
    let number = 1;
    let message1 = "hello";
    println!("message number {}: {}", number, message1);

    /*
     * multivariabel
     */
    let (angka, kata) = (23, "Yo");
    println!("angkanya : {0}", angka);
    println!("katanya : {0}", kata);

    /*
     * multivariabel dengan didefinisikan tipenya
     */
    let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 4);
    println!("var5: {0}", var5);
    println!("var6: {0}", var6);
    var6 = 24;
    println!("var6: {0}", var6);
    println!("var7: {0}", var7);
}
