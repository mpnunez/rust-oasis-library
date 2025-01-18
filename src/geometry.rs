trait PointTrait {
    type CoordinateType;
    fn x(&self) -> Self::CoordinateType;
    fn y(&self) -> Self::CoordinateType;
    fn set_x(&mut self, _: Self::CoordinateType) -> ();
    fn set_y(&mut self, _: Self::CoordinateType) -> ();
}

struct Point {
    x: i64,
    y: i64,
}

impl PointTrait for Point {
    type CoordinateType = i64;
    fn x(&self) -> Self::CoordinateType {self.x}
    fn y(&self) -> Self::CoordinateType {self.y}
    fn set_x(&mut self, x: Self::CoordinateType) -> () {self.x = x}
    fn set_y(&mut self, y: Self::CoordinateType) -> () {self.y = y}
}

struct Rectangle<PointType: PointTrait> {
    pt1: PointType,
    pt2: PointType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_point(){
        let p = Point { x: 0, y: 0 };
        assert_eq!(p.x(),0);
        assert_eq!(p.y(),0);
    }

}