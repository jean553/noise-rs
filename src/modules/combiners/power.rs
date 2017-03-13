// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use modules::NoiseModule;
use num_traits::Float;

/// Noise module that raises the output vaule from the first source module
/// to the power of the output value of the second source module.
pub struct Power<Source1, Source2> {
    /// Outputs a value.
    pub source1: Source1,

    /// Outputs a value.
    pub source2: Source2,
}

impl<Source1, Source2> Power<Source1, Source2> {
    pub fn new(source1: Source1, source2: Source2) -> Power<Source1, Source2> {
        Power {
            source1: source1,
            source2: source2,
        }
    }
}

impl<Source1, Source2, T, U> NoiseModule<T> for Power<Source1, Source2>
    where Source1: NoiseModule<T, Output = U>,
          Source2: NoiseModule<T, Output = U>,
          T: Copy,
          U: Float,
{
    type Output = U;

    fn get(&self, point: T) -> Self::Output {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}