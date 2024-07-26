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

use mdoc::*;

fn main() {
    let mut m = Mdoc::new(
        None,
        DocumentTitle {
            title: title! {"test"},
            section: section! { "1" },
            arch: None,
        },
        name! { "name" },
        description! { "This is a description in one line." },
        None,
    );
    m.add_section(
        "synopsis",
        std::iter::once(Line::Text(vec![roman(
            "The mandoc utility formats manual pages for display.",
        )])),
    );

    // let lines = mdoc::mdoc! {
    //     { roman("mplasfdsafasm df asdf amf asmfasdm fmas mms fasd") } ; Line::NAME.clone() ; roman("mfasdfa sdfa."),
    //     roman("lorem ipsum")
    // };
    // m.lines.extend(lines.into_iter());
    println!("{}", m.render());
}
