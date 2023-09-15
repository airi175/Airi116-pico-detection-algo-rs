mod tree;

use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read};

use image::{GenericImageView, Luma};

use crate::geometry::Square;
us