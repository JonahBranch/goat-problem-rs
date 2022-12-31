pub struct Circle { 
    x_pos: f64,
    radius: f64,
}

impl Circle {
    pub fn new(x_pos: f64, radius: f64) -> Self {
        Circle { x_pos, radius }
    }

    pub fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    pub fn area_overlap(&self, other: &Circle) -> f64 {
        let (bigger, smaller) = match self.radius > other.radius {
            true => (self, other),
            false => (other, self),
        };

        let d = f64::abs(bigger.x_pos - smaller.x_pos);
        if d >= bigger.radius + smaller.radius { return 0.0; }
        if d <= bigger.radius - smaller.radius { return smaller.area(); }

        let d1 = (bigger.radius.powi(2) - smaller.radius.powi(2) + d.powi(2))/(2.0 * d);
        let d2 = d - d1;

        let a1 = bigger.radius.powi(2) * f64::acos(d1 / bigger.radius) 
            - d1 * (bigger.radius.powi(2) - d1.powi(2)).sqrt();
        let a2 = smaller.radius.powi(2) * f64::acos(d2 / smaller.radius)
            - d2 * (smaller.radius.powi(2) - d2.powi(2)).sqrt();

        a1 + a2
    }    
}

#[cfg(test)]
mod tests {
    use crate::Circle;

    #[test]
    fn no_overlap() {
        let c1 = Circle::new(0.0, 1.0);
        let c2 = Circle::new(3.0, 1.0);
        let result = c1.area_overlap(&c2);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn total_overlap() {
        let c1 = Circle::new(0.0, 3.0);
        let c2 = Circle::new(1.0, 1.0);
        let result = c1.area_overlap(&c2);
        assert_eq!(result, c2.area());
    }

    #[test]
    fn partial_overlap() {
        let c1 = Circle::new(0.0, 3.0);
        let c2 = Circle::new(2.0, 3.0);
        let result = c1.area_overlap(&c2);
        assert_eq!(true, result > 0.0);
        assert_eq!(true, result < c1.area());
    }
}