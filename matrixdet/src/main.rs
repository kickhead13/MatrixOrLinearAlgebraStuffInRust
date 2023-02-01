static DETERR: &str = "THERE IS NO INVERSE OF THIS MATRIX";

pub struct Mat{
    pub height: u16,
    pub width: u16,
    pub body: Vec<Vec<i64>>,
}

impl Mat {
    fn new(width: u16, height: u16) -> Self {
        let this: Vec<Vec<i64>> = vec![vec![0i64; width.into()]; height.into()];
        return Self {
            height: height,
            width: width,
            body: this
        };
    }
    
    fn printmat(&self) {
        self.body
            .iter()
            .for_each(|v| println!("{:?}", v));
        println!();
    }

    fn minor(&self, i: u16, j: u16) -> Self {
        let mut minor: Self = Self::new(self.height-1,
                                        self.width-1);
        let mut it: usize = 0;
        let mut jt: usize = 0;
        for x in 0..self.height {
            jt = 0;
            for y in 0..self.width {
                if x != i && y != j {
                    minor.body[if it > i.into() {it-1} else{it}]
                         [if jt > j.into() {jt-1} else{jt}] =
                         self.body[x as usize][y as usize];
                }
                jt += 1;
            }
            it += 1;
        }
        return minor;
    }

    fn det(&self) -> i64 {
        if self.height == 1 {return self.body[0][0];}
        if self.height == 2 {
            return self.body[0][0]*self.body[1][1] -
                    self.body[1][0]*self.body[0][1];
        }
        let mut sum: i64 = 0;
        for x in 0..self.width {
            if self.body[0][x as usize] != 0 {
                sum += self.body[0][x as usize] *
                    (if (x + 2) % 2 == 0 {1} else {-1}) *
                    self.minor(0,x).det();
            }
        }
        return sum;
    }

    fn trans(&self) -> Self {
        let mut trans: Self = Self::new(self.width,
                                self.height);
        for x in 0..self.height {
            for y in 0..self.width {
                trans.body[y as usize][x as usize] =
                    self.body[x as usize][y as usize];
            }
        }
        return trans;
    }

    fn inverse(&self) -> Option<Self>{
        //returns the inverse multiplied by the determinant of the original
        let mut star: Self = self.trans();
        let mut detM = self.det();
        if detM != 0 {
            for x in 0..self.height {
                for y in 0..self.width {
                    star.body[x as usize][y as usize] =
                        (if (x+y+2)%2 == 0 {1} else {-1}) *
                        self.trans().minor(x,y).det();
                }
            }
            return Some(star);
        }
        return None;
    }

    fn multiply(&self, other: Self) -> Self {
        let mut mul: Self = Self::new(self.height, self.width);

        for x in 0..self.height {
            for y in 0..self.width {
                for k in 0..self.height {
                    mul.body[x as usize][y as usize] +=
                        self.body[k as usize][y as usize]*
                        other.body[x as usize][k as usize]; 
                }
            }
        }
        return mul;
    }
}

fn main() {
    let mut m: Mat = Mat::new(5, 5);
    m.body = vec![vec![2, 1, 1, 0, 4],
                  vec![6, 3, 0, 0, 4],
                  vec![6, 3, 1, 3, 4],
                  vec![2, 1, 0, 1, 4],
                  vec![4, 3, 0, 2, 3]];
    println!("{}", m.det());
    m.trans().printmat();
    match m.inverse() {
        Some(a) => {a.printmat()},
        None => {},
    }
    m.multiply(m.inverse().expect(DETERR)).printmat();
}
