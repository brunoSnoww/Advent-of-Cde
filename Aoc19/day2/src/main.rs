fn main() {
    let mut input = include_str!("./input.txt")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec::<i32>>();
    for noun in (0..100) {
        for verb in (0..100) {
            let mut input = input.clone();
            let mut idx = 0;
            let l = input.len();
            input[1] = noun;
            input[2] = verb;
            while idx < l {
                match input[idx] {
                    1 | 2 => {
                        let x = input[input[idx + 1] as usize];
                        let y = input[input[idx + 2] as usize];
                        let slot = input[idx + 3];
                        let res = if input[idx] == 1 { x + y} else { x * y };
                        input[slot as usize] = res;
                        idx += 4;
                    },
                    99 => {
                        break;
                    },
                    _ => unreachable!()
                }
            }
            if input[0] == 19690720 {
                let noun = input[1];
                let verb = input[2];
                println!("{}",100*noun + verb);
            }
        }
    }


}
