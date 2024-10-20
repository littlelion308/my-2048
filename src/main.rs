use std::io;
use rand::Rng;

const DEBUG: bool = true;

fn main() {
   reneral_loop(); // loop and exept input
}

fn reneral_loop() { //constant loop

    let mut play_field: [u64; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];


    gen_tiles_playfield(&mut play_field);
    gen_tiles_playfield(&mut play_field);


    draw(&mut play_field);
    loop{

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line!");
    let guess:i64 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess <= 9 && guess >0 { // check if in range
        process_input(&mut play_field, guess);
        debug_assert!(true,"in range");
    } else if guess == 0  { // end if zero
        break;
    } else {
        continue;
    }

    draw(&play_field);
    if DEBUG { println!("{}",&guess);}

    }
}

fn process_input( mut field: &mut [u64; 16], input:i64) {

    if input == 1 {
        move_left(&mut field);
        //draw(&mut field);
        merge_left(&mut field);
        move_left(&mut field);
    }
    else if input == 2{
        move_down(&mut field);
        merge_down(&mut field);
        move_down(&mut field);
    }
    else if input == 3{
        move_up(&mut field);
        merge_up(&mut field);
        move_up(&mut field);
    }
    else if input == 4{
        move_right(&mut field);
        merge_right(&mut field);
        move_right(&mut field);
    }
    gen_tiles_playfield(&mut field);
}

fn gen_tiles_playfield(field: &mut [u64; 16]) {

    let mut counter = 0;
    'loop_play_tile: loop {
        let number = rand::thread_rng().gen_range(0..=15);

        if field[number] == 0 {
            let chance_number = rand::thread_rng().gen_range(1..=10);

            if chance_number == 10 {
                field[number] = 4;
            } else {
                field[number] = 2;
            }
            if DEBUG {println!("replace {number} with {}", &field[number]);}
            break;
        }

        counter += 1;
        if counter > 30 {
            for number in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15] {
                if field[number] == 0 {
                    let chance_number = rand::thread_rng().gen_range(1..=10);

                    if chance_number == 10 {
                        field[number] = 4;
                    } else {
                        field[number] = 2;
                    }
                    if DEBUG {println!("replace {number} with {}", &field[number]);}
                    break 'loop_play_tile;
                }
            }
            println!("You Lose!!");
            break; // should end game
        }
    }
    if DEBUG {println!("took {counter} tries");}
}
fn draw(field: &[u64; 16]) {


    { // row one
        let zero: u64 = field[0];
        let one: u64 = field[1];
        let tow: u64 = field[2];
        let three: u64 = field[3];

    println!("{zero}, {one}, {tow}, {three}");

    };
    { // row tow
        let zero: u64 = field[4];
        let one: u64 = field[5];
        let tow: u64 = field[6];
        let three: u64 = field[7];

    println!("{zero}, {one}, {tow}, {three}");

    };
    { // row three
        let zero: u64 = field[8];
        let one: u64 = field[9];
        let tow: u64 = field[10];
        let three: u64 = field[11];

    println!("{zero}, {one}, {tow}, {three}");

    };
    { // row four
        let zero: u64 = field[12];
        let one: u64 = field[13];
        let tow: u64 = field[14];
        let three: u64 = field[15];

    println!("{zero}, {one}, {tow}, {three}");

    }

}

fn move_left(field: &mut [u64; 16]) { // 1
    for a in [1,5,9,13] {
        'cont: for b in [0,1,2] {
            let c = a + b; // loop over every row, then column, but skip the first tile
            let mut d = 0;
            while field[c-d-1] == 0 && field[c-d] != 0 { // check repeatedly whether neighboring
                                                         // tile is empty
                let e = c - d;
                field[e-1] = field[e]; //move
                field[e] = 0;
                if DEBUG {println!( "check_move moved {e}, a={a}" );}
                if e <= 1 || e -1 == a - 1{ // end the check
                    continue 'cont;
                }
                d += 1;
            }
        }
    }
}
fn move_down(field: &mut [u64; 16]) { // 2
    for a in [8,4,0] {
        'cont: for b in [0,1,2,3] {
            let c = a + b; // loop over every row, then column, but skip the last row of tiles
            let mut d = 0;
            while field[c+d+4] == 0 && field[c+d] != 0 { // check repeatedly whether neighboring
                                                         // tile is empty
                let e = c + d;
                field[e+4] = field[e]; //move
                field[e] = 0;
                if DEBUG {println!( "check_move moved {e}, a={a}" );}
                if e >= 15 || e + 4 >= 12 { // end the check
                    continue 'cont;
                }
                d += 4;
            }
        }
    }
}
fn move_up(field: &mut [u64; 16]) { // 3
    for a in [4,8,12] {
        'cont: for b in [0,1,2,3] {
            let c = a + b; // loop over every row, then column, but skip the last row of tiles
            let mut d = 0;
            while field[c-d-4] == 0 && field[c-d] != 0 { // check repeatedly whether neighboring
                                                         // tile is empty
                let e = c - d;
                field[e-4] = field[e]; //move
                field[e] = 0;
                if DEBUG {println!( "check_move moved {e}, a={a}, d={d}, c={c}" );}
                if e <= 0 || e - 4 <= 3 { // end the check
                    continue 'cont;
                }
                d += 4;
            }
        }
    }
}
fn move_right(field: &mut [u64; 16]) { // 4
    for a in [3,7,11,15] {
        'cont: for b in [1,2,3] {
            let c = a - b; // loop over every row, then column, but skip the first tile
            let mut d = 0;
            while field[c+d+1] == 0 && field[c+d] != 0 { // check repeatedly whether neighboring
                                                         // tile is empty
                let e = c + d;
                field[e+1] = field[e]; //move
                field[e] = 0;
                if DEBUG {println!( "check_move moved {e}, a={a}" );}
                if e >= 15 || e + 1 >= a - 1 { // end the check
                    continue 'cont;
                }
                d += 1;
            }
        }
    }
}
fn merge_left(field: &mut [u64; 16]) { // 1
    for a in [0,1,2] {
        for b in [0,4,8,12] {
            let c = a + b;
            if field[c] == field[c+1] && field[c] != 0 {
                field[c] = field[c] + field[c+1];
                field[c+1] = 0;
                if DEBUG {println!("merged {c} left")}
            }
        }
    }
}
fn merge_down(field: &mut [u64; 16]) { // 2
    for a in [12,8,4] {
        for b in [0,1,2,3] {
            let c = a + b;
            if field[c] == field[c-4] && field[c] != 0 {
                field[c] = field[c] + field[c-4];
                field[c-4] = 0;
                if DEBUG {println!("merged {c} down")}
            }
        }
    }
}
fn merge_up(field: &mut [u64; 16]) { // 3
    for a in [0,4,8] {
        for b in [0,1,2,3] {
            let c = a + b;
            if field[c] == field[c+4] && field[c] != 0 {
                field[c] = field[c] + field[c+4];
                field[c+4] = 0;
                if DEBUG {println!("merged {c} up")}
            }
        }
    }
}
fn merge_right(field: &mut [u64; 16]) { // 4
    for a in [3,2,1] {
        for b in [0,4,8,12] {
            let c = a + b;
            if field[c] == field[c-1] && field[c] != 0 {
                field[c] = field[c] + field[c-1];
                field[c-1] = 0;
                if DEBUG {println!("merged {c} right")}
            }
        }
    }
}
