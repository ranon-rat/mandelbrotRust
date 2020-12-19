#[allow(non_upper_case_globals)]
use num::complex::Complex;


static width :i64=70;
static height :i64=70;
fn scale( int:i64,in_min:i64,in_max:i64,out_min:f64,out_max :f64)->f64{
    let n :f64=((int-in_min) as f64) /((in_max-in_min) as f64);
    let out :f64= n*(out_max-out_min)+out_min;
    return out
}
fn mandelbrot(){
        for px in 1..width{
            for py in 1..height{
                let cx :f64=scale(px, 0, width, -1.511, 1.0);
                let cy :f64=scale(py, 0, height, -1.0, 1.0);
                let c=Complex::new(cx,cy);
                let mut z= Complex::new(0.0,0.0);
                let mut i :i64=0;
                while z.norm()<2.0*2.0 && i<80{
                    z=z.powf(2.0)+c;
                    i+=1;
                }
                if i<80{
                    print!("#")
                }else{
                    print!(" ")
                }
            }
            println!()
        }
}
fn main(){
    mandelbrot();
}

