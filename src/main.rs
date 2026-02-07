fn main() {
    // vec![値;個数] [1;3]->[1,1,1]
    let mut onejigen: Vec<i32> = vec![0; 25];
    let mut row = 0;
    let width = 5;
    let mut column = 0;
    let mut zahyou = 0;
    let mut rowview;

    for _ in 0..onejigen.len() {
        //onejigen[zahyou] += 1;
        if row!=0&&row!=onejigen.len()/5-1&&column!=0&&column!=width-1{
            
                onejigen[zahyou] += 1;
            
        } 
        rowview = &onejigen[row*5..zahyou+1];
        column += 1;
        if column == 5 {
            println!("{:?}",rowview);
            row += 1;
            column = 0;
        }
        zahyou = row * width + column;
    }
}
