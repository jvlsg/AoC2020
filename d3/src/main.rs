//open squares (.) and trees (#)
//the same pattern repeats to the right many times:
//start by counting all the trees you would encounter for the slope right 3, down 1:
use std::fs;
use ndarray::prelude::*;

#[derive(Debug)]
enum PositionType{
    Open,
    Tree
}

///Struct area - vector of vectors of positions
struct Area{
    pos: Array2<PositionType>
}


//movement vector X,Y
#[derive(Debug, Copy, Clone)]
struct Vector2d{
    x:isize,
    y:isize
}

impl Vector2d{
    fn new (x:isize,y:isize) -> Self{
        Vector2d{x,y}
    }
}

impl std::ops::Add for Vector2d {
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


fn find_collisions(start: Vector2d,heading: Vector2d, area: Area) -> usize{
    let mut curr_pos = start;
    let ncols = area.pos.ncols() as isize;
    let mut count = 0;
    while curr_pos.y <= ncols{
        curr_pos = curr_pos + heading;
        match area.pos[[ curr_pos.x,curr_pos.y ]]{
            
        }
    }

    0
}

fn main() {

    let area = Area{
        pos: array![
            [PositionType::Open, PositionType::Tree],
            [PositionType::Open, PositionType::Open]]
    };
    println!("{:#?}",area.pos[[0,0]]);
}

#[cfg(test)]
mod tests{
    use super::*;    
    #[test]
    fn four_sq(){
        let area = Area{
            pos: array![
                [PositionType::Open, PositionType::Tree],
                [PositionType::Open, PositionType::Open]]
        };
        assert_eq!(find_collisions(
            Vector2d::new(0,0),
            Vector2d::new(1,1),
            area),1);
    }
}


