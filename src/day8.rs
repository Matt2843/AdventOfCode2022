use itertools::Itertools;

fn is_visible(arr: &[Vec<u32>], (x, y): (usize, usize)) -> bool {
    if x == 0 || x == arr.len() - 1 || y == 0 || y == arr[0].len() - 1 {
        return true;
    }
    let value = arr[x][y];
    if arr[0..x].iter().all(|row| row[y] < value) {
        return true;
    }
    if arr[(x + 1)..].iter().all(|row| row[y] < value) {
        return true;
    }
    if arr[x][0..y].iter().all(|&v| v < value) {
        return true;
    }
    if arr[x][(y + 1)..].iter().all(|&v| v < value) {
        return true;
    }
    return false;
}

fn scenic_score(arr: &[Vec<u32>], (x,y): (usize, usize)) -> usize {
    let tree_height = arr[x][y];
    let mut north_score = (0..x).rev()
        .take_while(|i| arr[*i][y] < tree_height).count();
    let mut south_score = (x+1..arr.len())
        .take_while(|i| arr[*i][y] < tree_height).count();
    let mut west_score = (0..y).rev()
        .take_while(|j| arr[x][*j] < tree_height).count();
    let mut east_score = (y+1..arr[x].len())
        .take_while(|j| arr[x][*j] < tree_height).count();
    if let Some(_) = (0..x).rev().find(|i| arr[*i][y] >= tree_height) { north_score += 1 }
    if let Some(_) = (x+1..arr.len()).find(|i| arr[*i][y] >= tree_height) { south_score += 1 }
    if let Some(_) = (0..y).rev().find(|j| arr[x][*j] >= tree_height) { west_score += 1 }
    if let Some(_) = (y+1..arr[x].len()).find(|j| arr[x][*j] >= tree_height) { east_score += 1 }
    north_score * south_score * west_score * east_score
}

pub fn solve(str: &str) -> (usize, usize) {
    let arr = str
        .lines()
        .map(|l| { l.chars()
            .map(|c| c.to_digit(10).expect("failed to convert"))
            .collect_vec()
        }).collect_vec();
    let visible_trees = arr
        .iter()
        .enumerate()
        .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, _)| (x, y)))
        .filter(|e| is_visible(&arr, *e))
        .count();
    let scenic_max = arr
        .iter()
        .enumerate()
        .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, _)| (x, y)))
        .map(|e| scenic_score(&arr, e))
        .max().unwrap();
    (visible_trees, scenic_max)
}