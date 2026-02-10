struct MapInfo {
    map_size: Vec<usize>,
    bomb_offset: Vec<Vec<usize>>,
    hint_num: Vec<Vec<usize>>,
}

impl MapInfo {
    fn new(base_size: Vec<usize>, base_percent: usize) -> Self {
        fn map_builder(size: &[usize], offset: &[Vec<usize>]) -> Vec<Vec<usize>> {
            let mut map: Vec<Vec<usize>> = vec![vec![0; size[0]]; size[1]]; //map=vec![vec![0; x10]; y12]

            for offset_yx in offset.iter() {
                map[offset_yx[1]][offset_yx[0]] += 9;
            }
            /*
            for i in 0..offset.len() {
                let offset_y = offset[i][1];
                let offset_x = offset[i][0];

                    if map[offset_y][offset_x] <= 8 && offset_y == 0&&offset_x == 0 {
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if map[offset_y][offset_x] <= 8 && offset_y == 0 {
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x-1] += 1;
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if offset_y == 0 && offset_x == size[0]-1 {
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y-1][offset_x-1] += 1;
                        map[offset_y-1][offset_x] += 1;
                    }
                    if map[offset_y][offset_x] <= 8 && offset_x == 0 {
                        map[offset_y-1][offset_x] += 1;
                        map[offset_y-1][offset_x+1] += 1;
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if map[offset_y][offset_x] <= 8 && offset_x == size[0]-1 {
                        map[offset_y-1][offset_x-1] += 1;
                        map[offset_y-1][offset_x] += 1;
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y+1][offset_x-1] += 1;
                        map[offset_y+1][offset_x] += 1;
                    }
                    if map[offset_y][offset_x] <= 8 && offset_y == size[1]-1 && offset_x == 0 {
                        map[offset_y+1][offset_x]+=1;
                        map[offset_y+1][offset_x+1]+=1;
                        map[offset_y][offset_x+1]+=1;
                    }
                    if map[offset_y][offset_x] <= 8 && offset_y == size[1]-1 {
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y-1][offset_x+1] +=1;
                        map[offset_y][offset_x-1] +=1;
                        map[offset_y][offset_x+1] +=1;
                    }
                    if map[offset_y][offset_x] <= 8 && offset_y == size[1]-1 && offset_x == size[0]-1{
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y][offset_x-1] +=1;
                    }
                    if map[offset_y][offset_x] <= 8 && offset_x != 0&& offset_y != 0 && offset_x != size[0]-1 && offset_y != size[1] -1 {
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y-1][offset_x+1] +=1;
                        map[offset_y][offset_x-1] +=1;
                        map[offset_y][offset_x+1] +=1;
                        map[offset_y+1][offset_x-1] +=1;
                        map[offset_y+1][offset_x] +=1;
                        map[offset_y+1][offset_x+1] +=1;
                    }
            } */

            /*
            for i in 0..offset.len() {
                let y = offset[i][1] + 1;
                let x = offset[i][0] + 1;

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
            } */
            for i in 0..offset.len() {
                let offset_x = offset[i][0];
                let offset_y = offset[i][1];
                if map[offset_y][offset_x] >= 9 {
                    map[offset_y][offset_x] = 9;
                }
            }
            /*
            for y in 0..size[0] + 2 {
            for x in 0..size[1] + 2 {
            if y == 0 || y == size[0] + 1 || x == 0 || x == size[1] + 1 {
            map[x].remove(0);
            }
            }
            }*/

            /*println!("// bomb and bomb offset");
            println!("------------");
            for y in map.iter() {
                for x in y.iter() {
                    print!("{}", x)
                }
                println!()
            }
            println!("------------");*/
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
                            print!("[true!{}/{}]",offset.len(),num_of_bomb);
                            offset_bool.push(true);
                        }
                        if offset[i][0] == gate_x && offset[i][1] == gate_y&&offset.len() != num_of_bomb {
                            print!("[false!{}/{}]",offset.len(),num_of_bomb);
                            offset_bool.push(false);
                        }

                }
                if offset_bool.iter().all(|&bool|bool) == true {
                    println!("[push!{}/{}]",offset.len(),num_of_bomb);
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
    let map_size: Vec<usize> = vec![4, 4];
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
    //let input_x = 0; //test
    //let input_y = 4; //test
    // render test
    for y in mapinfo.iter() {
        for x in y.iter() {
            print!("{}", x)
        }
        println!()
    }
    for y in map.iter() {
        for x in y.iter() {
            print!("{}", x)
        }
        println!()
    }
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
