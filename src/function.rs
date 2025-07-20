use crate::skeleton::{Cell,Sheet,Celltype};
use std::cmp::{min,max};
use std::time::Duration;
use std::thread;

// Changes from the original check_cycle in the excel in C:
// No need for linked list ; with two stacks one for dfs completion and one for dfs uncompleted we can maintain the toposort
// and check the cycle.
// Improvements:
// Though still O(m+n) , no need for linked stack with added butter rust compilation.
fn MASTER(id:usize,mat:&mut Sheet)->i32{
    let max_col = mat.cols;
    let index_1 = mat.matrix[id].cell1.unwrap_or(-1);
    let index_2 = mat.matrix[id].cell2.unwrap_or(-1);
    let from_col = (index_1 )%(max_col as i32);
    let to_col = (index_2 )%(max_col as i32);
    let from_row = (index_1)/(max_col as i32);
    let to_row = index_2/(max_col as i32);
    // if func_type == 0 { // Constant assignment
    //     mat.matrix[id].val = mat.matrix[id].op_val;
    //     return 1;
    // }
    match mat.matrix[id].kind{
    Celltype::Constant=>{
        mat.matrix[id].val = mat.matrix[id].op_val.unwrap();
        return 1;
    },
    Celltype::Arithmetic('+') =>{
            if mat.matrix[id].cell1.unwrap_or(-1)==-1{
                if (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false{
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val + mat.matrix[id].op_val.unwrap();
                    mat.matrix[id].is_valid=true;
                }
            }
            else if mat.matrix[id].cell2.unwrap_or(-1)==-1{
                if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).is_valid == false{
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).val + mat.matrix[id].op_val.unwrap();
                    mat.matrix[id].is_valid=true;
                }
            }
            else{
                if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).is_valid==false || (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false{
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).val + (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val;
                    mat.matrix[id].is_valid=true;
                }
            }
            return 1;
        },
        Celltype::Arithmetic('-') =>{
            if mat.matrix[id].cell2.unwrap_or(-1)==-1{
                if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).is_valid==false {
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1)as usize]).val - mat.matrix[id].op_val.unwrap();
                    mat.matrix[id].is_valid=true;
                }
                
            }
            else if mat.matrix[id].cell1.unwrap_or(-1)==-1 {
                if(mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false{
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= mat.matrix[id].op_val.unwrap()- (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val ;
                    mat.matrix[id].is_valid=true;
                }
            }
            else{
                if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).is_valid==false || (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false {
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).val - (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val;
                    mat.matrix[id].is_valid=true;
                }
            }
            return 1;
        },
        Celltype::Arithmetic('*')=>{
            if mat.matrix[id].cell1.unwrap_or(-1)==-1{
                if (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false{
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val * mat.matrix[id].op_val.unwrap();
                    mat.matrix[id].is_valid=true;
                }
            }
            else if mat.matrix[id].cell2.unwrap_or(-1)==-1 {
                if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).is_valid==false{
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1)as usize]).val * mat.matrix[id].op_val.unwrap();
                    mat.matrix[id].is_valid=true;
                }
            }
            else{
                if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).is_valid==false || (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false{
                    mat.matrix[id].is_valid=false;
                }
                else{
                    mat.matrix[id].val= (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1)as usize]).val * (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val;
                    mat.matrix[id].is_valid=true;
                }
            }
            return 1;
        },
        Celltype::Arithmetic('/')=>{
            if mat.matrix[id].cell2.unwrap_or(-1)==-1 {
                if mat.matrix[id].op_val.unwrap()==0{
                    mat.matrix[id].is_valid=false;
                    return 1;
                }
                else{
                    if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1)as usize]).is_valid==false{
                        mat.matrix[id].is_valid=false;
                    }
                    else{
                        mat.matrix[id].val= ((mat.matrix[mat.matrix[id].cell1.unwrap_or(-1)as usize]).val)/(mat.matrix[id].op_val.unwrap());
                        mat.matrix[id].is_valid=true;
                    }
                    
                }
                
                
            }
            else if mat.matrix[id].cell1.unwrap_or(-1)==-1{
                if (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val==0{
                    mat.matrix[id].is_valid=false;
                    return 1;
                }
                else{
                    if (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false{
                        mat.matrix[id].is_valid=false;
                    }
                    else{
                        mat.matrix[id].val= (mat.matrix[id].op_val.unwrap())/((mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val);
                        mat.matrix[id].is_valid=true;
                    }
                }

            }
            else{
                if (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val==0{
                    mat.matrix[id].is_valid=false;
                    return 1;
                }
                else{
                    if (mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).is_valid==false || (mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).is_valid==false{
                        mat.matrix[id].is_valid=false;
                    }
                    else{
                        mat.matrix[id].val= ((mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize]).val)/((mat.matrix[mat.matrix[id].cell2.unwrap() as usize]).val);
                        mat.matrix[id].is_valid=true;
                    }
                }
                
            }
            return 1;
        },
    Celltype::MIN=>{ // MIN(RANGE)
        MIN( from_row,from_col,to_row,to_col,mat,id);
        
        return 1;

    },
    Celltype::MAX=>{ // MAX(RANGE)

        MAX( from_row,from_col,to_row,to_col,mat,id);
        return 1;

    },

    Celltype::AVG=> { // AVG(RANGE)
        AVG( from_row,from_col,to_row,to_col,mat,id);
        return 1;
    },

    Celltype::SUM=> { // SUM(RANGE)

        SUM( from_row,from_col,to_row,to_col,mat, id);

        return 1;

    },

    Celltype::STDEV=> { // STDEV(RANGE)
        STDEV( from_row,from_col,to_row,to_col,mat,id);
        return 1;
    }

    Celltype::SLEEP=>{ // SLEEP(RANGE)
        SLEEP(id,mat);
        return 1;     
    }

    _=>{
        return 0;

    }
    }
    
}


fn MAX(f_r:i32,f_c:i32,t_r:i32,t_c:i32,mat:&mut Sheet,id:usize){
    let mut mx = i32::MIN;
    for i in f_r..=t_r{
        for j in f_c..=t_c{
            if mat.matrix[i as usize*mat.cols as usize + j as usize].is_valid == false{
                mat.matrix[id].is_valid = false;
                return;
            }
            mx = std::cmp::max(mx,mat.matrix[i as usize*mat.cols as usize + j as usize].val);
        }
    }
    mat.matrix[id].val = mx;
    mat.matrix[id].is_valid = true;
    return;

}

fn MIN(f_r:i32,f_c:i32,t_r:i32,t_c:i32,mat:&mut Sheet,id:usize){
    let mut mn = i32::MIN;
    for i in f_r..=t_r{
        for j in f_c..=t_c{
            if mat.matrix[i as usize*mat.cols as usize+ j as usize].is_valid == false{
                mat.matrix[id].is_valid = false;
                return;
            }
            mn = std::cmp::min(mn,mat.matrix[i as usize*mat.cols as usize + j as usize].val);
        }
    }
    mat.matrix[id].val = mn;
    mat.matrix[id].is_valid = true;
    return;

}

fn AVG(f_r:i32,f_c:i32,t_r:i32,t_c:i32,mat:&mut Sheet,id:usize){
    
    let mut sum = 0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if mat.matrix[i as usize*mat.cols as usize + j as usize].is_valid == false{
                mat.matrix[id].is_valid = false;
                return;
            }
            sum += mat.matrix[i as usize*mat.cols as usize + j as usize].val;
        }
    }

    let num_elements:i32 = ( t_r - f_r + 1 )*( t_c - f_c + 1 );
    mat.matrix[id].val = sum/num_elements;
    mat.matrix[id].is_valid = true;
    return;

}

fn SUM(f_r:i32,f_c:i32,t_r:i32,t_c:i32,mat:&mut Sheet,id:usize){
    
    let mut sum:i32 = 0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if mat.matrix[i as usize*mat.cols as usize + j as usize].is_valid == false{
                mat.matrix[id].is_valid = false;
                return;
            }
            sum += mat.matrix[i as usize*mat.cols as usize + j as usize].val;
        }
    }
    mat.matrix[id].val = sum;
    mat.matrix[id].is_valid = true;
    return;

}

fn STDEV(f_r:i32,f_c:i32,t_r:i32,t_c:i32,mat:&mut Sheet,id:usize){
    
    let mut sum:i64 = 0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            if mat.matrix[i as usize*mat.cols as usize + j as usize].is_valid == false{
                mat.matrix[id].is_valid = false;
                return;
            }
            sum += mat.matrix[i as usize*mat.cols as usize + j as usize].val as i64;
        }
    }
    let num_elements:f64 = ( t_r as f64 - f_r as f64 + 1.0 )*( t_c as f64 - f_c as f64+ 1.0 );
    let mean = (sum as f64)/num_elements;
    let mut var:f64 = 0.0;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            var += (mat.matrix[i as usize*mat.cols as usize + j as usize].val as f64-mean)*(mat.matrix[i as usize*mat.cols as usize + j as usize].val as f64-mean);
        }
    }
    var = var/num_elements;
    mat.matrix[id].val = var.round() as i32;
    mat.matrix[id].is_valid = true;
    return;

}

fn SLEEP(id:usize,mat:&mut Sheet){
    if(mat.matrix[id].cell1.unwrap_or(-1) == -1){
        let sec : i32 = mat.matrix[id].op_val.unwrap();
        assert!(sec>0,"Seconds can't be negative");
        thread::sleep(Duration::from_secs(sec as u64));
        mat.matrix[id].val = sec;
        mat.matrix[id].is_valid = true;
        return;
    }
    else{
        if mat.matrix[mat.matrix[id].cell1.unwrap_or(-1 )as usize].is_valid == false{
            mat.matrix[id].is_valid = false;
            return;
        }
        let sec : i32 = mat.matrix[mat.matrix[id].cell1.unwrap_or(-1) as usize].val;
        thread::sleep(Duration::from_secs(sec as u64));
        mat.matrix[id].val = sec;
        mat.matrix[id].is_valid = true;
        return;
    }
}

fn CHECK_CYCLE(id:usize ,vis:&mut Vec<bool>,mat:&mut Sheet,cell1:Option<i32>,cell2:Option<i32>,flag:&mut bool,t :i32,stack:&mut Vec<u32>){
    let mut st:Vec<u32> = Vec::new();
    let mut last_unvisited:Vec<usize> = vec![0;(mat.cols as usize)*(mat.rows as usize) ];
    st.push(mat.matrix[id].id);
    while st.len() > 0 {

        let id:usize = st[st.len()-1] as usize;
        let curr : &Cell = &mat.matrix[id];

        if vis[id]==false{
            if t > 1 && t < 7 {
                if (id / mat.cols as usize) >= (cell1.unwrap_or(-1) as usize/ mat.cols as usize) && (id / mat.cols as usize) <= (cell2.unwrap_or(-1)as usize / mat.cols as usize) {
                    if (id % mat.cols as usize) >= (cell1.unwrap_or(-1)as usize % mat.cols as usize) && (id % mat.cols as usize) <= (cell2.unwrap_or(-1) as usize% mat.cols as usize){
                        *flag = true;
                        
                    }
                }
            } else if t == 1 {
                if cell2.unwrap_or(-1) == -1&& cell1.unwrap_or(-1) == -1{
                    if (id / mat.cols as usize) == (cell1.unwrap_or(-1) as usize / mat.cols as usize) && (id % mat.cols as usize) == (cell1.unwrap_or(-1)  as usize% mat.cols as usize) {
                        *flag = true;
                    }
                    if (id / mat.cols as usize) == (cell2.unwrap_or(-1) as usize/ mat.cols as usize) && (id % mat.cols as usize) == (cell2.unwrap_or(-1)as usize % mat.cols as usize) {
                        *flag = true;
                    }
                } else if (cell1.unwrap_or(-1) == -1) {
                    if (id / mat.cols as usize) == (cell1.unwrap_or(-1) as usize/ mat.cols as usize) && (id % mat.cols as usize) == (cell1.unwrap_or(-1) as usize% mat.cols as usize) {
                        *flag = true;
                    }
                } else {
                    if (id / mat.cols as usize) == (cell2.unwrap_or(-1) as usize/ mat.cols as usize) && (id % mat.cols as usize) == (cell2.unwrap_or(-1)as usize % mat.cols as usize) {
                        *flag = true;
                    }
                }
            } else if t == 7 {
                if cell1.unwrap_or(-1) == -1{
                    if (id / mat.cols as usize) == (cell1.unwrap_or(-1) as usize / mat.cols as usize) && (id % mat.cols as usize) == (cell1.unwrap_or(-1) as usize % mat.cols as usize) {
                        *flag = true;
                    }
                }
            }
        }
        vis[id]=true;
        if *flag{
            return;
        }

        let mut lu:usize = last_unvisited[id];
        while lu<curr.out_neighbors.len(){
            if !vis[curr.out_neighbors[lu] as usize] {
                break;
            }
            lu+=1;
        }
        if last_unvisited[id] == curr.out_neighbors.len(){
            stack.push(st.pop().unwrap());
            continue;
        }
        last_unvisited[id] = lu;
        st.push(curr.out_neighbors[lu]);
    }
}

fn recalculate_node(mat:&mut Sheet , stack:&mut Vec<i64>){
    
    while(stack.len() > 0){
        let id:usize = stack.pop().unwrap() as usize;
        MASTER(id,mat); 
    }
}