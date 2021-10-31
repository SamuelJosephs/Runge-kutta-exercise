

pub mod methods{
pub fn calcTau(halflife:f64)->f64{
    let two:f64 = 2_f64;
    return halflife/two.ln();
}

pub fn f_rad(N:f64,halflife:f64)->f64{
    return -N/calcTau(halflife);
}

pub fn Euler(f:fn(f64)->f64,n0: f64, t0: f64, dt:f64,n_panels:i32,tmax:f64)->Vec<f64>{
    let mut output = Vec::<f64>::with_capacity((n_panels+1) as usize);
    output.push(n0);
    let dt = tmax/n_panels as f64;
    

    for i in 1..(n_panels+1){
        output.push(output[(i-1) as usize]+dt*f(output[(i-1) as usize]));
    }
    return output;
}

pub fn solve_rk4(f:fn(f64)->f64,n0:f64,t0:f64,dt:f64,nsteps:i32,tmax:f64)->Vec<f64>{ // tmax = t0 + dt*nsteps -> t
    let mut array = Vec::<f64>::with_capacity((nsteps+1) as usize);
    array.push(n0);
    let dt =  tmax/nsteps as f64;
    
    for i in 1..(nsteps+1){
        let k_1 = f(array[(i-1) as usize]);
        let y_2 = array[(i-1) as usize] + 0.5*dt*k_1;
        let k_2 = f(y_2);
        let y_3 = array[(i-1) as usize]+0.5*dt*k_2;
        let k_3 = f(y_3);
        let y_4 = y_3+0.5*dt*k_3;
        let k_4 = f(y_4);
        let k = (1 as f64/6 as f64)*(k_1+2.0*k_2+2.0*k_3+k_4);
        let y_final = array[(i-1) as usize] + dt*k;
        array.push(y_final);

    }
    return array;
}

}
