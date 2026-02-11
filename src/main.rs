struct MapInfo {
    map_size: Vec<usize>,
    bomb_offset: Vec<Vec<usize>>,
    hint_num: Vec<Vec<usize>>,
}

impl MapInfo {
    fn new(base_size: Vec<usize>, base_percent: usize) -> Self {
        fn map_builder(size: &[usize], offset: &[Vec<usize>]) -> Vec<Vec<usize>> {
            let mut map: Vec<Vec<usize>> = vec![vec![0; size[0]]; size[1]]; //map=vec![vec![0; x10]; y12]

            for i in 0..offset.len() {
                let offset_y = offset[i][1];
                let offset_x = offset[i][0];

                if size[0] <= 2 && size[1] <= 2{
                    if offset_y == 0&&offset_x == 0 && size[0] == 1 && size[1] == 2 {
                        map[offset_y+1][offset_x] += 1;
                    }
                    if offset_y == 1&&offset_x == 0 && size[0] == 1 && size[1] == 2 {
                        map[offset_y-1][offset_x] += 1;
                    }
                    if offset_y == 0&&offset_x == 0 && size[0] == 2 && size[1] == 1 {
                        map[offset_y][offset_x+1] += 1;
                    }
                    if offset_y == 0&&offset_x == 1 && size[0] == 2 && size[1] == 1 {
                        map[offset_y][offset_x-1] += 1;
                    }
                } else {
                    if offset_y == 0&&offset_x == 0 && size[0] >= 2 && size[1] >= 2{
                        map[offset_y][offset_x+1] += 1;/////migi
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if offset_y == 0 && offset_x != 0 && offset_x != size[0]-1 {
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x-1] += 1;
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if offset_y == 0 && offset_x == size[0]-1 && size[0] >= 2 && size[1] >= 2{
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y+1][offset_x-1] += 1;
                        map[offset_y+1][offset_x] += 1;
                    }
                    if offset_x == 0 && offset_y != 0 && offset_y != size[1]-1{
                        map[offset_y-1][offset_x] += 1;
                        map[offset_y-1][offset_x+1] += 1;/////migi
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x] += 1;///////hidari
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if offset_x == size[0]-1 && offset_y != 0 && offset_y != size[1]-1{
                        map[offset_y-1][offset_x-1] += 1;
                        map[offset_y-1][offset_x] += 1;
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y+1][offset_x-1] += 1;///////hidari
                        map[offset_y+1][offset_x] += 1;
                    }
                    if offset_y == size[1]-1 && offset_x == 0 && size[0] >= 2 && size[1] >= 2{
                        map[offset_y-1][offset_x]+=1;
                        map[offset_y-1][offset_x+1]+=1;
                        map[offset_y][offset_x+1]+=1;
                    }
                    if offset_y == size[1]-1 && offset_x != 0 && offset_x != size[0]-1{
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y-1][offset_x+1] +=1;
                        map[offset_y][offset_x-1] +=1;
                        map[offset_y][offset_x+1] +=1;
                    }
                    if offset_y == size[1]-1 && offset_x == size[0]-1&& size[0] >= 2 && size[1] >= 2{
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y][offset_x-1] +=1;
                    }
                    if offset_x != 0&& offset_y != 0 && offset_x != size[0]-1 && offset_y != size[1] -1 {
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y-1][offset_x+1] +=1;
                        map[offset_y][offset_x-1] +=1;
                        map[offset_y][offset_x+1] +=1;
                        map[offset_y+1][offset_x-1] +=1;
                        map[offset_y+1][offset_x] +=1;
                        map[offset_y+1][offset_x+1] +=1;
                    }

                }

            }

            for i in 0..offset.len() {
                let offset_x = offset[i][0];
                let offset_y = offset[i][1];
                    map[offset_y][offset_x] = 9;
            }
            map
        }
        fn set_offset_random(size: &[usize], percent: usize) -> Vec<Vec<usize>> {
            let mut offset: Vec<Vec<usize>> = vec![];
            let num_of_bomb = (size[0] * size[1]) * percent / 100;
            let rand_x = fastrand::usize(0..size[0]);
            let rand_y = fastrand::usize(0..size[1]);
            offset.push(vec![rand_x, rand_y]);

            while offset.len() != num_of_bomb {
                let mut offset_bool:Vec<bool> = vec![];
                let gate_x = fastrand::usize(0..size[0]);
                let gate_y = fastrand::usize(0..size[1]);
                for i in 0..offset.len() {
                        if offset[i][0] != gate_x && offset[i][1] != gate_y&&offset.len() != num_of_bomb {
                            offset_bool.push(true);
                        }
                        if offset[i][0] == gate_x && offset[i][1] == gate_y&&offset.len() != num_of_bomb {
                            offset_bool.push(false);
                        }

                }
                if offset_bool.iter().all(|&bool|bool) == true {
                    offset.push(vec![gate_x, gate_y])
                }
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
    let map_size: Vec<usize> = vec![10, 12];
    let bomb_per: usize = 50;
    let map_info = MapInfo::new(map_size, bomb_per);
    let rendered_map = render_map(&map_info);
    check_answer(&map_info, rendered_map);
}

fn render_map(map_info: &MapInfo) -> Vec<Vec<String>> {
    let map_size = &map_info.map_size;
    let render_size_x = map_size[0] + 2;
    let render_size_y = map_size[1] + 2;

    let mut play_map = vec![vec![" 🟦 ".to_string(); render_size_x]; render_size_y];

    for y in 0..render_size_y {
        for x in 0..render_size_x {
            if y == 0 || y == render_size_y - 1 || x == 0 || x == render_size_x - 1 {
                play_map[y][x] = " 🧱 ".to_string()
            } // Vec<Vec<String>>とVec<Vec<usize>>でpopの後の処理が変わるため処理を考える
        } // String -pop> '""' usize -pop> ''
    }// そもそもstringはpopしないようにする
    /* //render
    for y in play_map.iter() {
        for x in y.iter() {
            print!("{}", x)
        }
        println!()
    } */

    play_map
}

fn check_answer(map_info: &MapInfo, rendered_map: Vec<Vec<String>>) {
    let mapinfo = &map_info.hint_num;
    let mut map = rendered_map;
    let input_x = 0; //test
    let input_y = 0; //test
    // render test
    for y in mapinfo.iter() {
        for x in y.iter() {
            print!("{}", x)
        }
        println!()
    }

    println!("{},{}は{}でした！",input_x,input_y,mapinfo[input_y][input_x]);

        map[input_y+1][input_x+1] = num_convert(mapinfo[input_y][input_x]);

    for y in map.iter() {
        for x in y.iter() {
            print!("{}", x)
        }
        println!()
    }
}

fn num_convert(number: usize) -> String {
    match number {
        1 => "１".to_string(),
        2 => "２".to_string(),
        3 => "３".to_string(),
        4 => "４".to_string(),
        5 => "５".to_string(),
        6 => "６".to_string(),
        7 => "７".to_string(),
        8 => "８".to_string(),
        9 => "💣".to_string(),
        _ => "".to_string()
    }
}