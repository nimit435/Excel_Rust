use crate::skeleton::{cell,sheet};

fn MASTER(node:cell,mat:sheet)

fn MAX(f_r:i64,f_c:i64,t_r:i64,t_c:i64,mat:sheet,node:cell){
    
    assert!(f_r>=0&&f_r<mat.rows,"Starting row {f_r} is out of bounds" );
    assert!(t_r>=0&&t_r<mat.rows,"Ending row {t_r} is out of bounds" );
    assert!(f_r<=t_r,"Starting row {f_r} is greater than Ending row {t_r}" );
    assert!(f_c>=0&&f_c<mat.cols,"Starting collumn {f_c} is out of bounds" );
    assert!(t_c>=0&&t_c<mat.cols,"Ending collumn {t_c} is out of bounds" );
    assert!(f_c<=t_c,"Starting collumn {f_c} is greater than Ending collumn {t_c}" );
    let mut mx = i64::MIN;
    for i in f_r..=t_r{
        for j in f_c..=f_r{
            mx = cmp::max(mx,mat.matrx[i][j]);
        }
    }
}