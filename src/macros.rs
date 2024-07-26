//
// mdoc
//
// Copyright 2024 Emmanouil Pitsidianakis <manos@pitsidianak.is>
//
// This file is part of mdoc.
//
// mdoc is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// mdoc is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with mdoc. If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later

// use super::*;

#[macro_export]
macro_rules! mdoc {
    (line control=>$($stuff:expr)*) => {{
        $($stuff)*
    }};
    (line $($stuff:expr)*) => {{
        Line::Text(vec![$($stuff)*])
    }};
    ($( $($($stuff:tt)*);+ ),+$(,)?) => {{ // Vec<Line>
vec![
        $(
            $crate::mdoc!(line $($($stuff)*)*)
        ),*
]

    }};
}
