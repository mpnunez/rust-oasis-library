trait PointTrait {
    type CoordinateType;
    fn x(&self) -> Self::CoordinateType;
    fn y(&self) -> Self::CoordinateType;
    fn set_x(&mut self, _: Self::CoordinateType) -> ();
    fn set_y(&mut self, _: Self::CoordinateType) -> ();
}

trait RectangleTrait {
    type CoordinateType;
    fn length(&self) -> Self::CoordinateType;
    fn width(&self) -> Self::CoordinateType;
    fn llx(&self) -> Self::CoordinateType;
    fn lly(&self) -> Self::CoordinateType;
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

impl<PointType: geometry::PointTrait> RectangleTrait for Rectangle<PointType impl geometry::PointTrait> {
    type CoordinateType = PointType::CoordinateType;
    fn length(&self) -> Self::CoordinateType {
        return 0;
    }
    fn width(&self) -> Self::CoordinateType {
        return 0;
    }
    fn llx(&self) -> Self::CoordinateType {
        return 0;
    }
    fn lly(&self) -> Self::CoordinateType {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_point(){
        let p = Point { x: 4, y: 5 };
        assert_eq!(p.x(),4);
        assert_eq!(p.y(),5);
    }

    /*
    #[test]
    fn make_rectangle(){
        let pt1 = Point { x: 0, y: 0 };
        let pt2 = Point { x: 4, y: 5 };
        let r = Rectangle {pt1: pt1, pt2: pt2};
        assert_eq!(r.length(),4);
        assert_eq!(r.width(),5);
    }
    */

}