struct MapInfo {
    bomb_offset: Vec<Vec<usize>>,
    hint_num: Vec<Vec<usize>>
}   

impl MapInfo {
    fn new( base_size: Vec<usize> , base_percent:usize ) -> Self {
        fn map_builder( size: &[usize], offset: &[Vec<usize>]) -> Vec<Vec<usize>> {
            let mut map: Vec<Vec<usize>> = vec![vec![0;size[0]];size[1]];
            for i in offset.iter(){
                let x = i[0];
                let y = i[1];
                
                map[x+1][y] +=1;
                map[x+1][y+1] +=1;
                map[x][y+1] +=1;
                map[x-1][y+1] +=1;
                map[x-1][y] +=1;
                map[x-1][y-1] +=1;
                map[x][y-1] +=1;
                map[x+1][y-1] +=1;
            }
            for i in offset.iter(){
                let x = i[0];
                let y = i[1];
                map[x][y] =9;
            }
            map
        }
        fn set_offset_random( size: &[usize], percent: usize ) -> Vec<Vec<usize>> {
            let mut offset: Vec<Vec<usize>> = vec![];
            for _ in 0..(size[0]*size[1])*percent/100 {
                offset.push(vec![fastrand::usize(0..size[0]),fastrand::usize(0..size[1])]);
            }
            offset
        }
        
        let set_bomb_offset = set_offset_random(&base_size, base_percent);
        let set_hint_num = map_builder(&base_size, &set_bomb_offset);
        Self {
            bomb_offset: set_bomb_offset,
            hint_num: set_hint_num
        }
    }
}

fn main() {
    let map_size: Vec<usize> = vec![10,10];
    let bomb_per: usize = 20;
    let map_info = MapInfo::new(map_size, bomb_per);
    
}





/*
fn old_bomb_set(map: &mut MapInfo) {
    for y in 0..map.ms_map.len(){
        for x in 0..map.ms_map[0].len(){
            if fastrand::i32(0..100) <= 20 {
                map.bomb_map[y][x] =1;
            }
        }
    }

    //for row in stage.iter() {
    //    println!("{:?}",row)
    //}
    //stage
}
fn old_render(map: &MapInfo) -> Vec<Vec<String>> {
    let mut x_size = map.ms_map.len();
    let mut y_size = map.ms_map[0].len();
    //print!("{:?}{:?}",x,y);
    x_size += 2;
    y_size += 2;
    let mut render_map: Vec<Vec<String>> = vec![vec!["🟦".to_string(); x_size]; y_size];

    for y in 0..y_size {
        for x in 0..x_size {
            if x == 0 || x == x_size - 1 || y == 0 || y == y_size - 1 {
                render_map[x][y] = "🧱".to_string();
            }
        }
    }

    let ms_x_size = map.ms_map.len();
    let ms_y_size = map.ms_map[0].len();

    let bomb = &map.bomb_map;

    for y in 0..ms_y_size {
        for x in 0..ms_x_size {
            if bomb[y][x] == 1 {
                render_map[y + 1][x + 1] = "💣".to_string()
            }
        }
    }

    for y in render_map.iter() {
        for x in y.iter() {
            print!("{}", x);
        }
        println!()
    }
    render_map
}
*/