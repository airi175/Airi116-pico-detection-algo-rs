mod tree;

use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read};

use image::{GenericImageView, Luma};

use crate::geometry::Square;
use crate::traits::Region;

use super::Detection;

use tree::DetectorTree;

/// Implements object detection using a cascade of decision tree classif