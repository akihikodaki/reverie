/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::gdbstub::{commands::*, hex::*};
use bytes::BytesMut;

#[derive(PartialEq, Debug)]
pub struct g;

impl ParseCommand for g {
    fn parse(bytes: BytesMut) -> Option<Self> {
        if bytes.is_empty() { Some(g) } else { None }
    }
}