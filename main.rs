
mod libs;
use crate::libs::methods::Euler;
use crate::libs::methods::calcTau;
use crate::libs::methods::solve_rk4;
use std::fs::File;
use std::io::prelude::*;
fn main() {

    fn analytic(n0:f64,t:f64)->f64{
        let TAU = calcTau(24110 as f64);
        let t = t;
        let a = -t/TAU;
        let b = a.exp();
        let c = n0*b;
        return c;
    }
    
    fn f_rad(N:f64)->f64{
        let TAU = calcTau(24110 as f64);
        return -N/&TAU;
    }
    // let trial_vec = Euler(f_rad,1000.0,0.0,60.0,10);
    // let trial_vec_2 = solve_rk4(f_rad,1000.0,0.0,60.0,10);

    // for i in trial_vec_2{
    //     println!("{}",i);
    // }


    // Next we want to for varying n work out the difference at a specific index between euler and rk4 method

    let mut difference_euler = Vec::<f64>::with_capacity((10000001) as usize);
    let mut difference_rk4 = Vec::<f64>::with_capacity((10000001) as usize);
    let mut steps_counter = Vec::<f64>::with_capacity((100000001) as usize);

    let t0 = 0.0;
    let tmax = 10000.0;
    
    for i in 25..10000{
        
        let dt = (tmax-t0)/(i as f64);
        
        let euler = Euler(f_rad, 1000.0, 0.0, dt, i,32768.0);
        let analytic = analytic(1000.0,32768.0);
        let rk4 = solve_rk4(f_rad, 1000.0, 0.0, dt, i,32768.0);
        
        let e_difference = analytic - euler[euler.len()-1];
        let r_difference = analytic - rk4[euler.len()-1];
        
        difference_euler.push(e_difference);
        difference_rk4.push(r_difference);
        steps_counter.push(i as f64);


    }

    let mut file = File::create("difference2.dat").expect("Failed to write to path");
    
    //file.write_all(b"\"euler\"");
    //file.write_all(b"\n");
    
    for i in 0..difference_euler.len(){
        let output:String = String::with_capacity(50);
        

        let output = output +&steps_counter[i].to_string() +" "+&difference_euler[i].to_string() + " "+&difference_rk4[i].to_string()+"\n";
        
        file.write_all(output.as_bytes());
    }    

    // file.write_all(b"\n");
    // file.write_all(b"\n");
    // file.write_all(b"\"rk4\"");
    // file.write_all(b"\n");
    // for i in 0..difference_euler.len(){
    //     let output:String = String::with_capacity(50);
        

    //     let output = output +&steps_counter[i].to_string() +" "+&difference_rk4[i].to_string() + "\n";
        
    //     file.write_all(output.as_bytes());
    // }    

}
