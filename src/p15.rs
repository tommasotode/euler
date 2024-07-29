fn explore(x : usize, y : usize, limit_x : usize, limit_y : usize, dp : &mut Vec<Vec<u64>>) -> u64 {
  
  if dp[x][y] != 0 {
    return dp[x][y];
  }

  if x + 1 > limit_x || y + 1 > limit_y {
    return 1;
  }

  dp[x][y] = explore(x+1, y, limit_x, limit_y, dp) + explore(x, y+1, limit_x, limit_y, dp);
  dp[x][y]

}

pub fn main() {
  let grid_size = 20;

  let mut dp = vec![vec![0; grid_size+1]; grid_size+1];
  let a = explore(0, 0, 20, 20, &mut dp);

  println!("{}", a);
}