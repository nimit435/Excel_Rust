use crate::skeleton::{cell,sheet};
use cmp::{min,max};
use std::time::Instant;

// Changes from the original check_cycle in the excel in C:
// No need for linked list ; with two stacks one for dfs completion and one for dfs uncompleted we can maintain the toposort
// and check the cycle.
// Improvements:
// Though still O(m+n) , no need for linked stack with added butter rust compilation.
fn MASTER(node:&mut cell,mat:&mut sheet){
    let max_col = mat.cols;
    let index_1 = node.cell1.unwrap_or(-1);
    let index_2 = node.cell2;
    let from_col = index_1%max_col;
    let to_col = index_2%max_col;
    let from_row = index_1/max_col;
    let to_row = index_2/max_col;
    if func_type == 0 { // Constant assignment
        node.val = node.op_val;
        return 1;
    }
    match node.kind{
    celltype::Arithmatic('+') =>{
            if node.cell1.unwrap_or(-1)==-1{
                if (sheet.matrix+ node.cell2).isValid==0{
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell2).val + node.op_val;
                    node.isValid=1;
                }
            }
            else if node.cell2==-1{
                if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0{
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell1.unwrap_or(-1)).val + node.op_val;
                    node.isValid=1;
                }
            }
            else{
                if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0 || (sheet.matrix+ node.cell2).isValid==0{
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell1.unwrap_or(-1)).val + (sheet.matrix+ node.cell2).val;
                    node.isValid=1;
                }
            }
            return 1;
        },
        celltype::Arithmatic('-') =>{
            if node.cell2==-1{
                if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0 {
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell1.unwrap_or(-1)).val - node.op_val;
                    node.isValid=1;
                }
                
            }
            else if node.cell1.unwrap_or(-1)==-1 {
                if(sheet.matrix+ node.cell2).isValid==0{
                    node.isValid=0;
                }
                else{
                    node.val= node.op_val- (sheet.matrix+ node.cell2).val ;
                    node.isValid=1;
                }
            }
            else{
                if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0 || (sheet.matrix+ node.cell2).isValid==0 {
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell1.unwrap_or(-1)).val - (sheet.matrix+ node.cell2).val;
                    node.isValid=1;
                }
            }
            return 1;
        },
        celltype::Arithmatic('*')=>{
            if node.cell1.unwrap_or(-1)==-1{
                if (sheet.matrix+ node.cell2).isValid==0{
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell2).val * node.op_val;
                    node.isValid=1;
                }
            }
            else if node.cell2==-1 {
                if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0{
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell1.unwrap_or(-1)).val * node.op_val;
                    node.isValid=1;
                }
            }
            else{
                if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0 || (sheet.matrix+ node.cell2).isValid==0{
                    node.isValid=0;
                }
                else{
                    node.val= (sheet.matrix+ node.cell1.unwrap_or(-1)).val * (sheet.matrix+ node.cell2).val;
                    node.isValid=1;
                }
            }
            return 1;
        },
        celltype::Arithmatic('/')=>{
            if node.cell2==-1 {
                if node.op_val==0{
                    node.isValid=0;
                    return 1;
                }
                else{
                    if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0{
                        node.isValid=0;
                    }
                    else{
                        node.val= ((sheet.matrix+ node.cell1.unwrap_or(-1)).val)/(node.op_val);
                        node.isValid=1;
                    }
                    
                }
                
                
            }
            else if node.cell1.unwrap_or(-1)==-1{
                if (sheet.matrix+ node.cell2).val==0{
                    node.isValid=0;
                    return 1;
                }
                else{
                    if (sheet.matrix+ node.cell2).isValid==0{
                        node.isValid=0;
                    }
                    else{
                        node.val= (node.op_val)/((sheet.matrix+ node.cell2).val);
                        node.isValid=1;
                    }
                }

            }
            else{
                if (sheet.matrix+ node.cell2).val==0{
                    node.isValid=0;
                    return 1;
                }
                else{
                    if (sheet.matrix+ node.cell1.unwrap_or(-1)).isValid==0 || (sheet.matrix+ node.cell2).isValid==0{
                        node.isValid=0;
                    }
                    else{
                        node.val= ((sheet.matrix+ node.cell1.unwrap_or(-1)).val)/((sheet.matrix+ node.cell2).val);
                        node.isValid=1;
                    }
                }
                
            }
            return 1;
        },
    celltype::MIN=>{ // MIN(RANGE)
        MIN( from_row,from_col,to_row,to_col,mat,node);
        
        return 1;

    },
    celltype::MAX=>{ // MAX(RANGE)

        MAX( from_row,from_col,to_row,to_col,mat,node);
        return 1;

    },

    celltype::AVG=> { // AVG(RANGE)
        AVG( from_row,from_col,to_row,to_col,mat,node);
        return 1;
    },

    celtype::SUM=> { // SUM(RANGE)

        SUM( from_row,from_col,to_row,to_col,mat,node);

        return 1;

    },

    celltype::STDEV=> { // STDEV(RANGE)
        STDEV( from_row,from_col,to_row,to_col,mat,node);
        return 1;
    }

    celltype::SLEEP=>{ // SLEEP(RANGE)
        SLEEP(node,mat);
        return 1;     
    }

    _=>{
        return 0;

    }
    }
    
}


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
    if(node.cell1.unwrap_or(-1) == -1){
        let sec : i64 = node.op_val;
        assert!(sec>0,"Seconds can't be negative");
        thread::sleep(Duration::from_secs(sec));
        node.val = sec;
        node.isValid = 1;
        return;
    }
    else{
        if(mat.matrix[node.cell1.unwrap_or(-1)].valid == 0){
            node.isValid = 0;
            return;
        }
        let sec : i64 = mat.matrix[node.cell1.unwrap_or(-1)].val;
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
            if !vis[curr.OutNeighbours[lu]] {
                break;
            }
            lu+=1;
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

fn recalculate_node(node:&mut cell , mat:&mut sheet , stack:&mut Vec<i64>){
    while(stack.len() > 0){
        let id:i64 = stack.pop();
        MASTER(mat.matrix[id],mat); 
    }
}