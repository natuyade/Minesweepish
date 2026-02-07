use fastrand::Rng;

fn main() {
    // vec![値;個数] [1;3]->[1,1,1]
    // 配列数とwidthでいくらでも変形可能に
    let mut onejigen: Vec<i32> = vec![0; 400];
    let mut row = 0;
    let width = 20;
    let mut column = 0;
    let mut zahyou = 0;
    let mut rowview;

    for _ in 0..onejigen.len() {
        // onejigen[zahyou] += 1;
        if row!=0&&row!=onejigen.len()/width-1&&column!=0&&column!=width-1{
            // fastrandで確率実装
            if fastrand::i32(0..100) <= 80{
                onejigen[zahyou] += 1;
            } else {
                onejigen[zahyou] += 2;
            }
        } 
        rowview = &onejigen[row*width..=zahyou];
        column += 1;
        if column == width {
            println!("{:?}",rowview);
            row += 1;
            column = 0;
        }
        zahyou = row * width + column;
    }
}
