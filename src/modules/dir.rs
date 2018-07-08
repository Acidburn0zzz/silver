use std::env;
use Segment;

pub fn dir_segment(segment: &mut Segment, _: &[&str]) {
    // TODO: support short names
    segment.value = env::current_dir().unwrap().to_str().unwrap().to_owned()
}