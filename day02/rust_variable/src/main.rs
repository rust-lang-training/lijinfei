fn main() {
    let x = 3;
    println!("x is {x}");
    {
        let x = x + 3;
        println!("x is {x}");
    }
    println!("x is {x}");


    let v: Vec<f64> = vec![0.0,2.12,12.2,2.2];
    let a: [f64; 4] = [0.2, 3.0, 2.2, 4.0];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    println!("{:?}", &sv[1..]);
    println!("{:?}", &sa[1..]);

    let s1 = "123213 
    123123
    123123
    1";
    let s2 = "123213 \
    123123\
    123123\
    1";
    println!("{s1}");
    println!("{s2}");

    let r1 = r"C:/Promig";
    let r2 = r###"12312323""23123"###;
    println!("{r1}");
    println!("{r2}");

    // 格式化输出
    println!("{:04}", 22);
    // 基本占位
    let x = 3;

    let y = 4;
    println!("x = {x}");
    println!("y = {y}");
    println!("{1} {0}", y, x);
    // 宽度
    // :前的参数默认为0
    // :后为格式化
    // /**
    //  * 
    //  *  format_string := text [ maybe_format text ] *
    //     maybe_format := '{' '{' | '}' '}' | format
    //     format := '{' [ argument ] [ ':' format_spec ] [ ws ] * '}'
    //     argument := integer | identifier
    //     format_spec := [[fill]align][sign]['#']['0'][width]['.' 
    //     precision]type
    //     fill := character
    //     align := '<' | '^' | '>'
    //     sign := '+' | '-'
    //     width := count
    //     precision := count | '*'
    //     type := '' | '?' | 'x?' | 'X?' | identifier
    //     count := parameter | integer
    //     parameter := argument '$'
    //  */
    // $  取变量或者下标
    println!("{0:5}!", "x");
    println!("{0:1$}! {:3}!", "x", 5);
    println!("{1:0$}!", 5, "x");


    println!("{:0>5}", "x");
    println!("{:0<5}", "x");
    println!("{:0^5}", "x");


    fn five() -> f64 {
        let x: f64 = 5.0;
        x
    }
    println!("{}", five());

    let condition = five();
    let v = if condition == 5.0 {
        println!("is True");
        5
    } else {
        println!("is False");
        6
    };

    println!("{v}");
    fn loop_fn() -> i32{
        let mut idx = 0;
        loop {
            idx+=1;
            if idx==10 {
                break idx * 3;
            }

        }
    }
    println!("{}", loop_fn())
}
