impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut vis = vec![false; rooms.len()];
        vis[0] = true;
        for &k in &rooms[0] {
            let k = k as usize;
            if !vis[k] {
                visit(&rooms, k, &mut vis)
            }
        }
        vis.into_iter().all(|x| x)
    }
}

fn visit(rooms: &Vec<Vec<i32>>, n: usize, vis: &mut Vec<bool>) {
    vis[n] = true;
    for &k in &rooms[n] {
        let k = k as usize;
        if !vis[k] {
            visit(rooms, k, vis);
        }
    }
}

struct Solution;
