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

use crate::*;

#[test]
fn test_escapes_leading_control_chars() {
    assert_eq!("foo\n\\&.bar\n\\&'yo", escape_leading_cc("foo\n.bar\n'yo"));
}

#[test]
fn test_render_roman() {
    let text = Mdoc::default().text([roman("foo")]).to_mdoc();
    assert_eq!(text, "foo\n");
}

#[test]
fn test_render_dash() {
    let text = Mdoc::default().text([roman("foo-bar")]).to_mdoc();
    assert_eq!(text, "foo\\-bar\n");
}

#[test]
fn test_render_italic() {
    let text = Mdoc::default().text([italic("foo")]).to_mdoc();
    assert_eq!(text, "\\fIfoo\\fR\n");
}

#[test]
fn test_render_bold() {
    let text = Mdoc::default().text([bold("foo")]).to_mdoc();
    assert_eq!(text, "\\fBfoo\\fR\n");
}

#[test]
fn test_render_text() {
    let text = Mdoc::default().text([roman("roman")]).to_mdoc();
    assert_eq!(text, "roman\n");
}

#[test]
fn test_render_text_with_leading_period() {
    let text = Mdoc::default().text([roman(".roman")]).to_mdoc();
    assert_eq!(text, "\\&.roman\n");
}

#[test]
fn test_render_text_with_newline_period() {
    let text = Mdoc::default().text([roman("foo\n.roman")]).to_mdoc();
    assert_eq!(text, "foo\n\\&.roman\n");
}
#[test]
fn test_render_line_break() {
    let text = Mdoc::default()
        .text([roman("roman"), Inline::LineBreak, roman("more")])
        .to_mdoc();
    assert_eq!(text, "roman\n.br\nmore\n");
}

#[test]
fn test_render_control() {
    let text = Mdoc::default()
        .control("foo".into(), ["bar", "foo and bar"])
        .to_mdoc();
    assert_eq!(text, ".foo bar \"foo and bar\"\n");
}
