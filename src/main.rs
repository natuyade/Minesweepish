struct MapInfo {
    map_size: Vec<usize>,
    bomb_offset: Vec<Vec<usize>>,
    hint_num: Vec<Vec<usize>>,
}

impl MapInfo {
    fn new(base_size: Vec<usize>, base_percent: usize) -> Self {
        fn map_builder(size: &[usize], offset: &[Vec<usize>]) -> Vec<Vec<usize>> {
            let mut map: Vec<Vec<usize>> = vec![vec![0; size[0] + 2]; size[1] + 2];
            
            for i in 0..offset.len() {
                let x = offset[i][1] + 1;
                let y = offset[i][0] + 1;

                if map[y][x] <= 8 {
                map[y + 1][x] += 1;
                map[y + 1][x + 1] += 1;
                map[y][x + 1] += 1;
                map[y - 1][x + 1] += 1;
                map[y - 1][x] += 1;
                map[y - 1][x - 1] += 1;
                map[y][x - 1] += 1;
                map[y + 1][x - 1] += 1;
                map[y][x] = 9;
                }
            }
            for i in 0..offset.len() {
                let x = offset[i][1] + 1;
                let y = offset[i][0] + 1;
                    map[y][x] = 9;
            }
            for y in 0..size[0] + 2 {
                for x in 0..size[1] + 2 {
                    if y == 0 || y == size[0] + 1 || x == 0 || x == size[1] + 1 {
                        map[x].pop();
                    }
                }
            }

            println!("// bomb and bomb offset");
            println!("------------");
            for y in map.iter() {
                for x in y.iter() {
                    print!("{}", x)
                }
                println!()
            }
            println!("------------");
            println!("{:?}",map[1]);
            map
        }
        fn set_offset_random(size: &[usize], percent: usize) -> Vec<Vec<usize>> {
            let mut offset: Vec<Vec<usize>> = vec![];
            for _ in 0..(size[0] * size[1]) * percent / 100 {
                let x = fastrand::usize(0..size[1]);
                let y = fastrand::usize(0..size[0]);
                offset.push(vec![x, y]);
            }
            offset
        }

        let set_bomb_offset = set_offset_random(&base_size, base_percent);
        let set_hint_num = map_builder(&base_size, &set_bomb_offset);
        Self {
            map_size: base_size,
            bomb_offset: set_bomb_offset,
            hint_num: set_hint_num,
        }
    }
}

fn main() {
    let map_size: Vec<usize> = vec![10, 10];
    let bomb_per: usize = 20;
    let map_info = MapInfo::new(map_size, bomb_per);
    let rendered_map = render_map(&map_info);
    check_answer(&map_info, rendered_map);
}

fn render_map(map_info: &MapInfo) -> Vec<Vec<String>> {
    let map_size = &map_info.map_size;
    let render_size_x = map_size[1] + 2;
    let render_size_y = map_size[0] + 2;

    let mut play_map = vec![vec!["🟦".to_string(); render_size_x]; render_size_y];

    for y in 0..render_size_y {
        for x in 0..render_size_x {
            if y == 0 || y == render_size_y - 1 || x == 0 || x == render_size_x - 1 {
                play_map[y][x] = "🧱".to_string()
            }
        }
    }
    //render
    for y in play_map.iter() {
        for x in y.iter() {
            print!("{}", x)
        }
        println!()
    }
    //周り削除
    for y in 0..render_size_y {
        for x in 0..render_size_x {
            if y == 0 || y == render_size_y - 1 || x == 0 || x == render_size_x - 1 {
                play_map[y][x].pop();
            }
        }
    }
    print!("{:?}",play_map[0]);
    play_map
}

fn check_answer( map_info: &MapInfo, rendered_map: Vec<Vec<String>>) {
    let mapinfo = &map_info.hint_num;
    let map = rendered_map;
    let input_x = 1;
    let input_y = 5;
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
