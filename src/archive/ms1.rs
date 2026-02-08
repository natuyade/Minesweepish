use std::io::*;
fn main() {
    // vec![値;個数] [1;3]->[1,1,1]
    // 配列数とwidthでいくらでも変形可能に
    let mut onejigen: Vec<usize> = vec![0; 100];
    let mut row = 0;
    let width = 10;
    let mut column = 0;
    let mut zahyou = 0;
    //let mut rowview;
    let mut kotae;

    for _ in 0..onejigen.len() {
        // onejigen[zahyou] += 1;
        if row != 0 && row != onejigen.len() / width - 1 && column != 0 && column != width - 1 {
            // fastrandで確率実装
            if fastrand::i32(0..100) <= 80 {
                onejigen[zahyou] += 1;
            } else {
                onejigen[zahyou] += 2;
            }
        }
        //rowview = &onejigen[row*width..=zahyou];
        column += 1;
        if column == width {
            //println!("{:?}",rowview);
            row += 1;
            column = 0;
        }
        zahyou = row * width + column;
    }
    let mut youranswer;
    let limit = width - 2;
    loop {
        for i in 0..width {
            kotae = &onejigen[width * i..width * i + width];
            println!("{:?}", kotae);
        }

        println!("Type you wanna search a coordinate 1~{}", limit);
        
        let mut keyinput_x;
        let mut keyinput_y;
        let mut key_x;
        let mut key_y;
        loop {
            keyinput_x = "0".to_string();
            keyinput_y = "0".to_string();

            print!("x(column): ");
            stdout().flush().unwrap();
            stdin().read_line(&mut keyinput_x).unwrap();
            key_x = keyinput_x.trim().parse::<usize>().unwrap_or(0);
            println!("{}",key_x);

            print!("y(row): ");
            stdout().flush().unwrap();
            stdin().read_line(&mut keyinput_y).unwrap();
            key_y = keyinput_y.trim().parse::<usize>().unwrap_or(0);
            println!("{}",key_y);
            
            if key_x * width + key_y <= onejigen.len() {
                youranswer = key_y * width + key_x;
                if onejigen[youranswer] == 0 {
                    println!("Please type 1~{}", limit);
                } else if onejigen[youranswer] == 1 {
                    println!("...You're a living!");
                    break;
                } else {
                    break;
                }
            } else {
                println!("Please type 1~{}", limit);
            }
        }
        if onejigen[youranswer] == 2 {
            break;
        }
    }
    println!("That was bomb!\\DOKAAAN/\n");

    println!("thankyou for playing!\nPress Enter key to close");
    stdin().read_line(&mut String::new()).unwrap();
}
