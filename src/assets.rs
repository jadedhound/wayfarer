#[rustfmt::skip]
pub const NAMES: [&str; 23] = ["Abigail","Emilia","Allison","Clara","Leah","Myla","Ryanna","Valerie","Bram","Abram","Astin","Bradyn","Cartus","Eric","Gavin","Han","Jax","Jovan","Liam","Remus","Sebastion","Xander","Havy"];

/// -2: 10%, -1: 20%, 0: 15%, +1: 25%, +2: 20%, +3: 10%
pub const ABILITY_MOD: [i32; 20] = [
    -2, -2, -1, -1, -1, -1, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3,
];

pub const LONG_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque erat nulla, porttitor non purus eu, sollicitudin ornare dui. Praesent pulvinar scelerisque bibendum. Maecenas lobortis viverra venenatis. Donec congue in augue id viverra. Proin pulvinar, odio sit amet euismod tristique, neque ipsum blandit tortor, varius iaculis ante diam at erat. Praesent erat tellus, imperdiet at est quis, malesuada lacinia lorem. Duis et pharetra odio, efficitur posuere justo. Curabitur quam augue, imperdiet cursus vestibulum quis, vulputate a tellus. Vivamus sit amet nibh non eros molestie aliquet.";
