/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::gdbstub::{commands::*, hex::*};

pub struct p {
    pub reg_id: usize,
}