pub mod grain_effect;
mod tone_curve;

pub trait FromTuple<F, D> {
    type Err;

    fn from_tuple(tuple: (F, D)) -> Result<Self, Self::Err> where Self: Sized;
}
