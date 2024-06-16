fn main() {
    println!("Number");
    // checked 要么Some 要么none
    // u8 256
    let sum = 10u8.checked_add(30);
    let sum2 = 244u8.checked_add(30);
    assert_eq!(sum.is_some(), true);
    assert_eq!(sum2.is_none(), true);
    // wrapping 超出时取余
    let sum3 =20u8.wrapping_add(200);
    let sum4 = 20u8.wrapping_add(249);
    println!("{:?}", sum3);
    println!("{:?}", sum4);
    // saturating 返回最大的值
    let sum5 = 20u8.saturating_add(200);
    let sum6 = 20u8.saturating_add(250);
    println!("{:?}", sum5);
    println!("{:?}", sum6);
    // overflowing  返回一个元组，第一个值是正常计算或者取余 第二个值是是否发生了溢出
    let sum7 = 20u8.overflowing_add(220);
    let sum8 = 20u8.overflowing_add(240);
    println!("{:?}", sum7);
    println!("{:?}", sum8);

    let v: Vec<f64> = vec![0.0, 0.0192, 0.2, 0.3];
    let s: [f64; 4] = [0.2, 0.3, 2.1, 3.2];


    let sv = &v;
    let ss = &s;
    println!("{:?}", &sv[0..2]);
    println!("{:?}", &ss[1..3]);
}
