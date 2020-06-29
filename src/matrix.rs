use std::time::{Duration,Instant};

struct Matrix4d {
    mat: [[f32;4]; 4]
}
impl Matrix4d {
    fn matrix_mul(a:&Matrix4d, b:&Matrix4d) -> Matrix4d {
        let mut matrix = Matrix4d {
            mat: [[0.,0.,0.,0.],
                  [0.,0.,0.,0.],
                  [0.,0.,0.,0.],
                  [0.,0.,0.,0.]]
        };
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    matrix.mat[i][j] += a.mat[i][k]*b.mat[k][j]; 
                }
            }
        }
        matrix

        }
    fn transpose(e:&Matrix4d)-> Matrix4d {
        let mut transpose_mat = Matrix4d {
            mat: [[0.,0.,0.,0.],
                  [0.,0.,0.,0.],
                  [0.,0.,0.,0.],
                  [0.,0.,0.,0.]]
        };
        for i in 0..4 {
            for j in 0..4 {
                transpose_mat.mat[i][j]=e.mat[j][i];
            }
        }
        transpose_mat
    }
    fn print(self) {
        for i in 0..4 {
            for j in 0..4 {
                print!("{} ", self.mat[i][j]);
            }
            print!("\n");
        }
    } 
    }
fn main() {
    let now = Instant::now();
    let a = Matrix4d {
        mat: [[1.,1.,1.,1.],
              [1.,1.,1.,1.],
              [1.,1.,1.,1.],
              [1.,1.,1.,1.]]
    };
    let b = Matrix4d {
        mat: [[1.,1.,0.,0.],
              [0.,1.,0.,0.],
              [0.,0.,1.,0.],
              [0.,0.,0.,1.]]
    };
    let d=Matrix4d::transpose(&b);
    d.print();
    for i in 0..100_000 {
        let c = Matrix4d::matrix_mul(&a, &b);
    }

    println!("elapsed:{}",now.elapsed().as_millis());
}