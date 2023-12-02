fn visible(map: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || j == map.len() - 1 || i == map.len() - 1 {
        return true;
    }

    let curr_height = map[i][j];

    return
        map[0..i].iter().all(|vec| curr_height > vec[j]) ||
        map[i+1.. ].iter().all(|vec| curr_height > vec[j]) ||
        map[i][0..j].iter().all(|height| curr_height > *height) ||
        map[i][j+1.. ].iter().all(|height| curr_height > *height);

}

fn scenic_score(map: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let mut scores = vec![0, 0, 0, 0];
    let treehouse_height: u32 = map[i][j];

    // Up
    for x in (0..i).rev() {
        if map[x][j] < treehouse_height {
            scores[0] += 1;
        } else if map[x][j] >= treehouse_height {
            scores[0] += 1;
            break;
        }
    }
    
    // Down
    for x in i+1..map.len() {
        if map[x][j] < treehouse_height {
            scores[1] += 1;
        } else if map[x][j] >= treehouse_height {
            scores[1] += 1;
            break;
        }
    }

    // Left
    for x in (0..j).rev() {
        if map[i][x] < treehouse_height {
            scores[2] += 1;
        } else if map[i][x] >= treehouse_height {
            scores[2] += 1;
            break;
        }
    }

    // Right
    for x in j+1..map.len() {
        if map[i][x] < treehouse_height {
            scores[3] += 1;
        } else if map[i][x] >= treehouse_height {
            scores[3] += 1;
            break;
        }
    }

    scores.iter().product()
}

pub fn process_part1(input: &str) -> String {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line
             .chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect())
        .collect();

    let mut total: u32 = 0;
    for i in 0..map.len() {
        for j in 0..map.len() {
            if visible(&map, i, j) {
                total += 1;
            }
        }
    }

    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line
             .chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect())
        .collect();

    let mut highest_score = 0;
    for i in 0..map.len() {
        for j in 0..map.len() {
            let score = scenic_score(&map, i, j);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "8");
    }
}
