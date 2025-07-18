use crate::skeleton::{cell,sheet};
use cmp::{min,max};
use std::time::Instant;
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

fn CHECK
