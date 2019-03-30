#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

impl<T> Rect<T>
where
    T: PartialOrd + std::ops::Add<Output = T> + Copy,
{
    pub fn get_upper_right(&self) -> Point<T> {
        return Point::new(self.x + self.width, self.y + self.height);
    }

    pub fn contains(&self, point: &Point<T>) -> bool {
        point.x > self.x
            && point.x < self.x + self.width
            && point.y > self.y
            && point.y < self.y + self.height
    }
}
