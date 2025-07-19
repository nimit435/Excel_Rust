use crate::skeleton::{cell,sheet};
use cmp::{min,max};
use std::time::Instant;

// Changes from the original check_cycle in the excel in C:
// No need for linked list ; with two stacks one for dfs completion and one for dfs uncompleted we can maintain the toposort
// and check the cycle.
// Improvements:
// Though still O(m+n) , no need for linked stack with added butter rust compilation.
fn MASTER(node:cell,mat:sheet)

fn MAX(f_r:i64,f_c:i64,t_r:i64,t_c:i64,mat:&mut sheet,node:&mut cell){
    
    assert!(f_r>=0&&f_r<mat.rows,"Starting row {f_r} is out of bounds" );
    assert!(t_r>=0&&t_r<mat.rows,"Ending row {t_r} is out of bounds" );
    assert!(f_r<=t_r,"Starting row {f_r} is greater than Ending row {t_r}" );
    assert!(f_c>=0&&f_c<mat.cols,"Starting collumn {f_c} is out of bounds" );
    assert!(t_c>=0&&t_c<mat.cols,"Ending collumn {t_c} is out of bounds" );
    assert!(f_c<=t_c,"Starting collumn {f_c} is greater than Ending collumn {t_c}" );
    let mut mx = i64::MIN;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if sheet.mat[i*mat.cols + j].isValid == 0{
                node.isValid = 0;
                return;
            }
            mx = cmp::max(mx,mat.matrix[i*mat.cols + j].val);
        }
    }
    node.val = mx;
    node.isValid = 1;
    return;

}

fn MIN(f_r:i64,f_c:i64,t_r:i64,t_c:i64,mat:&mut sheet,node:&mut cell){
    
    assert!(f_r>=0&&f_r<mat.rows,"Starting row {f_r} is out of bounds" );
    assert!(t_r>=0&&t_r<mat.rows,"Ending row {t_r} is out of bounds" );
    assert!(f_r<=t_r,"Starting row {f_r} is greater than Ending row {t_r}" );
    assert!(f_c>=0&&f_c<mat.cols,"Starting collumn {f_c} is out of bounds" );
    assert!(t_c>=0&&t_c<mat.cols,"Ending collumn {t_c} is out of bounds" );
    assert!(f_c<=t_c,"Starting collumn {f_c} is greater than Ending collumn {t_c}" );
    let mut mn = i64::MIN;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if sheet.mat[i*mat.cols + j].isValid == 0{
                node.isValid = 0;
                return;
            }
            mn = cmp::min(mx,mat.matrix[i*mat.cols + j].val);
        }
    }
    node.val = mn;
    node.isValid = 1;
    return;

}

fn AVG(f_r:i64,f_c:i64,t_r:i64,t_c:i64,mat:&mut sheet,node:&mut cell){
    
    assert!(f_r>=0&&f_r<mat.rows,"Starting row {f_r} is out of bounds" );
    assert!(t_r>=0&&t_r<mat.rows,"Ending row {t_r} is out of bounds" );
    assert!(f_r<=t_r,"Starting row {f_r} is greater than Ending row {t_r}" );
    assert!(f_c>=0&&f_c<mat.cols,"Starting collumn {f_c} is out of bounds" );
    assert!(t_c>=0&&t_c<mat.cols,"Ending collumn {t_c} is out of bounds" );
    assert!(f_c<=t_c,"Starting collumn {f_c} is greater than Ending collumn {t_c}" );
    let mut sum = 0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if sheet.mat[i*mat.cols + j].isValid == 0{
                node.isValid = 0;
                return;
            }
            sum += mat.matrix[i*mat.cols + j].val;
        }
    }

    let num_elements:i64 = ( t_r - f_r + 1 )*( t_c - f_c + 1 );
    node.val = sum/num_elements;
    node.isValid = 1;
    return;

}

fn SUM(f_r:i64,f_c:i64,t_r:i64,t_c:i64,mat:&mut sheet,node:&mut cell){
    
    assert!(f_r>=0&&f_r<mat.rows,"Starting row {f_r} is out of bounds" );
    assert!(t_r>=0&&t_r<mat.rows,"Ending row {t_r} is out of bounds" );
    assert!(f_r<=t_r,"Starting row {f_r} is greater than Ending row {t_r}" );
    assert!(f_c>=0&&f_c<mat.cols,"Starting collumn {f_c} is out of bounds" );
    assert!(t_c>=0&&t_c<mat.cols,"Ending collumn {t_c} is out of bounds" );
    assert!(f_c<=t_c,"Starting collumn {f_c} is greater than Ending collumn {t_c}" );
    let mut sum:i64 = 0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if sheet.mat[i*mat.cols + j].isValid == 0{
                node.isValid = 0;
                return;
            }
            sum += mat.matrix[i*mat.cols + j].val;
        }
    }
    node.val = sum;
    node.isValid = 1;
    return;

}

fn STDEV(f_r:i64,f_c:i64,t_r:i64,t_c:i64,mat:&mut sheet,node:&mut cell){
    
    assert!(f_r>=0&&f_r<mat.rows,"Starting row {f_r} is out of bounds" );
    assert!(t_r>=0&&t_r<mat.rows,"Ending row {t_r} is out of bounds" );
    assert!(f_r<=t_r,"Starting row {f_r} is greater than Ending row {t_r}" );
    assert!(f_c>=0&&f_c<mat.cols,"Starting collumn {f_c} is out of bounds" );
    assert!(t_c>=0&&t_c<mat.cols,"Ending collumn {t_c} is out of bounds" );
    assert!(f_c<=t_c,"Starting collumn {f_c} is greater than Ending collumn {t_c}" );
    let mut sum:i64 = 0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if sheet.mat[i*mat.cols + j].isValid == 0{
                node.isValid = 0;
                return;
            }
            sum += mat.matrix[i*mat.cols + j];
        }
    }
    let num_elements:f64 = ( t_r - f_r + 1 )*( t_c - f_c + 1 );
    let mean = sum/num_elements;
    let mut var:f64 = 0.0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            var += (mat.matrix[i*mat.cols + j].val-mean)*(mat.matrix[i*mat.cols + j].val-mean);
        }
    }
    var = var/num_elements;
    node.val = var.round();
    node.isValid = 1;
    return;

}

fn SLEEP(node:&mut cell,mat:&mut sheet){
    if(node.cell1 == -1){
        let sec : i64 = node.op_val;
        assert!(sec>0,"Seconds can't be negative");
        thread::sleep(Duration::from_secs(sec));
        node.val = sec;
        node.isValid = 1;
        return;
    }
    else{
        if(mat.matrix[node.cell1].valid == 0){
            node.isValid = 0;
            return;
        }
        let sec : i64 = mat.matrix[node.cell1].val;
        assert(sec>0,"Seconds can't be negative.");
        thread::sleep(Duration::from_secs(sec));
        node.val = sec;
        node.isValid = 1;
        return;
    }
}

fn CHECK_CYCLE(node:&cell ,vis:&mut Vec<bool>,mat:&mut sheet,cell1:i64,cell2:i64,flag:&mut bool,t :i32,stack:&mut Vec<i64>){
    let mut st:Vec<i64> = Vec::new();
    let mut last_unvisited:Vec<i64> = vec![0;mat.cols*mat.rows as usize];
    let mut complete :Vec<bool> = vec![false;mat.cols*mat.rows as usize];
    st.push(node.id);
    while st.len() > 0 {

        let id:i64 = st[st.len()-1];
        let curr : &cell = mat.matrix[id];

        if vis[id]==false{
            if t > 1 && t < 7 {
                if (id / mat.cols) >= (cell1 / mat.cols) && (id / mat.cols) <= (cell2 / mat.cols) {
                    if (id % mat.cols) >= (cell1 % mat.cols) && (id % mat.cols) <= (cell2 % mat.cols){
                        *flag = true;
                        
                    }
                }
            } else if t == 1 {
                if cell2 != -1 && cell1 != -1 {
                    if (id / mat.cols) == (cell1 / mat.cols) && (id % mat.cols) == (cell1 % mat.cols) {
                        *flag = true;
                    }
                    if (id / mat.cols) == (cell2 / mat.cols) && (id % mat.cols) == (cell2 % mat.cols) {
                        *flag = true;
                    }
                } else if (cell1 != -1) {
                    if (id / mat.cols) == (cell1 / mat.cols) && (id % mat.cols) == (cell1 % mat.cols) {
                        *flag = true;
                    }
                } else {
                    if (id / mat.cols) == (cell2 / mat.cols) && (id % mat.cols) == (cell2 % mat.cols) {
                        *flag = true;
                    }
                }
            } else if t == 7 {
                if cell1 != -1 {
                    if (id / mat.cols) == (cell1 / mat.cols) && (id % mat.cols) == (cell1 % mat.cols) {
                        *flag = true;
                    }
                }
            }
        }
        vis[id]=true;
        if *flag{
            return;
        }

        let mut lu:i64 = last_unvisited[id];
        while lu<curr.OutNeighbours.len(){
            if vis[curr.OutNeighbours[lu]] {
                if complete[curr.OutNeighbours[lu]] == false{
                    *flag = true;
                    return;
                }
                lu+=1;
            }
            else{
                break;
            }
        }
        if last_unvisited[id] == curr.OutNeighbours.len(){
            stack.push(st.pop().unwrap());
            complete[id] = true;
            continue;
        }
        last_unvisited[id] = lu;
        st.push(curr.OutNeighbours[lu]);
    }
}

