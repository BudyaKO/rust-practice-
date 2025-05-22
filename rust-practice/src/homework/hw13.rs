#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

#[derive(Clone, Copy)]
struct Rectangle { a: Point, b: Point }

impl Rectangle {
    fn normalized(&self) -> Rectangle {
        let left = self.a.x.min(self.b.x);
        let right = self.a.x.max(self.b.x);
        let top = self.a.y.max(self.b.y);
        let bottom = self.a.y.min(self.b.y);
        Rectangle { a: Point { x: left, y: top }, b: Point { x: right, y: bottom } }
    }

    fn contains(&self, p: Point) -> bool {
        let r = self.normalized();
        p.x >= r.a.x && p.x < r.b.x && p.y < r.a.y && p.y >= r.b.y
    }
}

fn area_occupied(rects: &[Rectangle]) -> i32 {
    let min_x = rects.iter().map(|r| r.a.x.min(r.b.x)).min().unwrap();
    let max_x = rects.iter().map(|r| r.a.x.max(r.b.x)).max().unwrap();
    let min_y = rects.iter().map(|r| r.a.y.min(r.b.y)).min().unwrap();
    let max_y = rects.iter().map(|r| r.a.y.max(r.b.y)).max().unwrap();

    let mut count = 0;
    for x in min_x..max_x {
        for y in min_y..max_y {
            let p = Point { x, y };
            if rects.iter().any(|r| r.contains(p)) {
                count += 1;
            }
        }
    }
    count
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ]
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Зайнята площа: {}", occupied);
}