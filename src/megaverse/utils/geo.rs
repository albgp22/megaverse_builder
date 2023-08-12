/*
    Geometry utilities package
*/

use itertools::Itertools;
use log;
use num::Integer;
use std::{
    error,
    fmt::{self, Debug, Display},
    hash::Hash,
    iter::Step,
    ops::Sub,
};

#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/*
    We define a custom error type for when the geometric figure cannot be constructed
    from imput data.
*/

#[derive(Debug, Clone)]
struct ImpossibleFigureErr;

impl fmt::Display for ImpossibleFigureErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Geometric figure impossible to construct")
    }
}
impl error::Error for ImpossibleFigureErr {}

/*
Absurdly type-generic function to return a generic iterator containing the coordinates for
the cross in a width x height grid. How many traits a generic integer must comply with? :O
*/
pub fn compute_cross_coordinates<T>(width: T, height: T) -> Result<impl Iterator<Item = (T, T)>>
where
    T: Integer + Sub + PartialOrd + Step + Clone + Copy + Hash + Debug + Display,
{
    let square_environment_size = width.min(height);
    if width != height {
        log::warn!(
            "Height ({height}u) and width ({width}u) do not match. \
            Considering it a ({square_environment_size}u x {square_environment_size}u) grid."
        )
    }
    if square_environment_size <= T::zero() {
        log::error!("Maps cannot have negative coordinates");
        Err(ImpossibleFigureErr.into())
    } else {
        let two = T::one() + T::one(); // Two is not a pre-defined trait
        Ok((two..square_environment_size - two)
            .flat_map(move |i| vec![(i, square_environment_size - i - T::one()), (i, i)])
            .unique())
    }
}
