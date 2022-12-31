use goat_problem::Circle;

/// return the percentage of the corral the goat can reach minus 0.5 
/// such that goat_problem(radius) == 0 is the answer to the goat problem
/// https://mathworld.wolfram.com/GoatProblem.html
fn goat_problem(radius: f64) -> f64 {
    let corral = Circle::new(0.0, 1.0);
    let goat = Circle::new(1.0, radius);

    (corral.area_overlap(&goat) / corral.area()) - 0.5
}

fn bisection_search(function: fn(f64) -> f64, left_bracket: f64, right_bracket: f64, error: f64) -> Option<f64> {
    let mut mid_result; 
    let mut previous_result = f64::INFINITY;
    let mut left_bracket = left_bracket;
    let mut right_bracket = right_bracket;
    
    if left_bracket > right_bracket { return None; }

    loop {
        let midpoint = (left_bracket + right_bracket) / 2.0;
        mid_result = function(midpoint);

        if previous_result == midpoint { return Some(midpoint); }
        if mid_result.abs() <= error / 2.0 { return Some(midpoint) }

        if mid_result * function(left_bracket) < 0.0 { 
            (left_bracket, right_bracket) = (left_bracket, midpoint);
        } 
        else if mid_result * function(right_bracket) < 0.0 { 
            (left_bracket, right_bracket) = (midpoint, right_bracket);
        }
        else { return None; }

        previous_result = midpoint;
        println!("guess: {}", midpoint);
    }
} 

fn main() {
    let result = bisection_search(goat_problem, 1.0, 2.0, f64::powi(10.0, -100));
    
    // should hopefully match this https://oeis.org/A133731/constant
    println!("The value of r is {:?}", result);
}
