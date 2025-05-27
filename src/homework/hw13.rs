use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point(i32, i32);

#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn covered_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for dx in 0..self.width {
            for dy in 0..self.height {
                points.push(Point(self.x + dx, self.y + dy));
            }
        }
        points
    }
}

fn calculate_occupied_area(rectangles: &[Rectangle]) -> usize {
    let mut occupied = HashSet::new();
    for rect in rectangles {
        for point in rect.covered_points() {
            occupied.insert(point);
        }
    }
    occupied.len()
}

fn main() {
    // Тестовий приклад
    let rectangles = vec![
        Rectangle { x: 1, y: 1, width: 3, height: 2 }, // покриває 6 клітинок
        Rectangle { x: 2, y: 2, width: 3, height: 2 }, // перекриває частково
    ];

    let area = calculate_occupied_area(&rectangles);
    println!("Загальна зайнята площа: {}", area);
}
