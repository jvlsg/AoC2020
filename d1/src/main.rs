//Find the two entries that sum to 2020;

use std::fs;
use std::str::FromStr;

fn two_eq_target(v: &Vec<i32>, target: i32){
    let mut i=0; //leftmost index
    let mut j=v.len()-1; //rightmost index
    //if i>j we iterated over the entire vector and couldn't find a pair
    while i<=j {
        if &v[i]+&v[j]==target {
            println!("{} + {} = {}",&v[i],&v[j],target);
            println!("{} * {} = {}",&v[i],&v[j],&v[i]*&v[j]);
            break;
        };
        // if the sum is less than target, we must increase the sum.
        // As the vector is ordered, this is done by getting a larger term
        // i.e. increasing the leftmost index.
        // else, we must decrease it by getting a smaller term, i.e. decrease 
        // the rightmost index.
        if &v[i]+&v[j]<target {i+=1} else {j-=1};
    }
}

fn three_eq_target(v: &Vec<i32>, target: i32){

    let mut i=0; //leftmost index
    let mut j=((v.len()-1)/2) as usize; // Central index
    let mut k=v.len()-1; //rightmost index

    //The value of the last sum. Used to avoid cyclical looping
    //between a movement and the last movement
    let mut last_sum = 0; 
    
    loop {
        //Break condition, stop if no room left for movements (i.e. all adjacent)
        if i+1 == j && j+1 == k {break};

        let sum = &v[i]+&v[j]+&v[k];
        if sum == target{
            println!("{} + {} + {} = {}",&v[i],&v[j],&v[k],target);
            println!("{} * {} * {} = {}",&v[i],&v[j],&v[k],&v[i]*&v[j]*&v[k]);
            break;
        } 
        //We must decrease sum: move j or k left
        else if sum > target {
            //Sum if we move j left
            //If the movement will make j overlap with i, we cannot move. 
            //Nor if the result equals the last sum (we would 'undo' the last movement)
            let mut dj = &v[i]+&v[j-1]+&v[k]; 
            if j-1 == i || dj == last_sum{dj = i32::MAX};

            let mut dk = &v[i]+&v[j]+&v[k-1];
            if k-1 == j || dk == last_sum{dk = i32::MAX};
            
            //We want to decrease as much as possible
            if dj < dk {j-=1} else {k-=1};
        }  
        //Analogous to above, now if sum < target
        else { 
            let mut di = &v[i+1]+&v[j]+&v[k];
            if i+1 == j && di == last_sum{di = 0};
            
            let mut dj = &v[i]+&v[j+1]+&v[k];
            if j+1 == k && di == last_sum{dj = 0};

            if di > dj {i+=1} else {j+=1};
        }
        last_sum = sum;
    }

}

fn main(){
    let mut input: Vec<i32> = fs::read_to_string("input/input.txt")
        .unwrap()
        .split_whitespace()
        .map(|x| i32::from_str(x).unwrap())
        .collect();
    input.sort();
    
    let target = 2020;   

    // PT.1 -------------------------------------------------------------------
    two_eq_target(&input,target);
    // PT.2 -------------------------------------------------------------------
    three_eq_target(&input,target);
}

