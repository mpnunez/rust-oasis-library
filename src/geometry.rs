use num_traits::PrimInt;

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

struct Point<Ct: PrimInt> {
    x: Ct,
    y: Ct,
}


impl<Ct: PrimInt> PointTrait for Point<Ct> {
    type CoordinateType = Ct;
    fn x(&self) -> Self::CoordinateType {self.x}
    fn y(&self) -> Self::CoordinateType {self.y}
    fn set_x(&mut self, x: Self::CoordinateType) -> () {self.x = x}
    fn set_y(&mut self, y: Self::CoordinateType) -> () {self.y = y}
}



struct Rectangle<PointType: PointTrait> {
    pt1: PointType,
    pt2: PointType,
}


impl<PointType: PointTrait<CoordinateType: PrimInt>> RectangleTrait for Rectangle<PointType> {
    type CoordinateType = PointType::CoordinateType;
    fn length(&self) -> Self::CoordinateType {
        return self.pt2.x() - self.pt1.x();
    }
    fn width(&self) -> Self::CoordinateType {
        return self.pt2.y() - self.pt1.y();
    }
    fn llx(&self) -> Self::CoordinateType {
        return self.pt1.x();
    }
    fn lly(&self) -> Self::CoordinateType {
        return self.pt1.y();
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