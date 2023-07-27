use std::ops::Add;

#[derive(Debug)]
struct Puple {
    cnt: i32,
    hight: f32,
}

impl Add for Puple {
    type Output = Puple;
    fn add(self, rhs: Self) -> Self::Output {
        Puple {
            cnt: self.cnt + rhs.cnt,
            hight: {if self.hight >= rhs.hight {
                        self.hight
                        }
                    else
                        {rhs.hight}
                    },
            }
    }
}

impl Puple {
    fn print_info(self) {
        println!("My cnt is {}, my height is {}.", self.cnt, self.hight);
    }
}

// fn call_puple(p: &dyn Add) -> Add::Output {
//     p.print_info();
// }

fn main() {
    let p1 = Puple {cnt: 1, hight: 1.0};
    let p2 = Puple {cnt: 2, hight: 2.0};
    let p3 = p1+p2;
    println!("{:?}", p3);

    let p4 = Puple {cnt: 10, hight: 12.0};
    p4.print_info();
}

