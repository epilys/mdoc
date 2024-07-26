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

//! A document in the **mdoc** format.
//!
//!  is a family of Unix text-formatting languages, implemented
//! > The [**mdoc**] language supports authoring of manual pages for the man(1) utility by allowing
//! > semantic annotations of words, phrases, page sections and complete manual pages. Such
//! > annotations are used by formatting tools to achieve a uniform presentation across all manuals
//! > written in [**mdoc**], and to support hyperlinking if supported by the output medium.
//!
//! [**mdoc**]: https://man.openbsd.org/mdoc.7

//#![warn(missing_docs)]

#[cfg(test)]
mod tests;

#[macro_use]
pub mod macros;

#[cfg(feature = "clap")]
pub mod from_clap;

use std::borrow::Cow;
use std::io::Write;
use std::write;

/// An **mdoc** document, consisting of lines.
///
/// Lines are either control lines (requests that are built in, or
/// invocations of macros), or text lines.
///
/// # Example
///
/// ```
/// # use mdoc::*;
/// let doc = Mdoc::default()
///     .control("TH", ["FOO", "1"])
///     .control("SH", ["NAME"])
///     .text([roman("foo - do a foo thing")])
///     .render();
/// assert!(doc.ends_with(".TH FOO 1\n.SH NAME\nfoo \\- do a foo thing\n"));
/// ```
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Mdoc {
    date: Option<DocumentDate>,
    title: DocumentTitle,
    os: Option<OperatingSystem>,
    name: Name,
    description: Description,
    synopsis: Vec<Line>,
    exit_status: Vec<Line>,
    return_values: Vec<Line>,
    examples: Vec<Line>,
    errors: Vec<Line>,
    files: Vec<Line>,
    environment: Vec<Line>,
    standards: Vec<Line>,
    see_also: Vec<Line>,
    history: Vec<Line>,
    authors: Vec<Line>,
    pub lines: Vec<Line>,
}

type Str = Cow<'static, str>;

macro_rules! macros {
    ($($(stuff:tt)?),*(,)?) => {
        $($($stuff)*)*
    };
    ($ident:tt, $({ $field:tt: $typ:ty, $type_ident:ident })*) => {
        $(macros!(def $field, $type_ident);)*

            #[derive(Default, PartialEq, Eq, Debug, Clone)]
            pub struct $ident {
                $(
                    pub $field: $typ,
                )*
            }
    };
    (as $subtype:tt) => {
        pub $field: Option<$subtype>
    };
    ($subtype:tt) => {
        pub $field: $subtype
    };
    (def $field:tt, $ident:tt) => {
        #[macro_export]
        macro_rules! $field {
            ($val:expr) => {
                $ident::new($val)
            }
        }

        #[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $ident(pub Str);

        impl $ident {
            pub fn new(s: impl Into<Str>) -> Self {
                Self(s.into())
            }
        }
    };
}

macros! {
    DocumentDate,
    { month: Month, Month }
    { day: Day, Day }
    { year: Year, Year }
}
macros! {
    DocumentTitle,
    { title: Title, Title }
    { section: Section, Section }
    { arch: Option<Arch>, Arch }

}
macros! {
    OperatingSystem,
    {system: System, System}
    {version: Option<Version>, Version}
}

macros! { def name, Name }
macros! { def description, Description }

impl Mdoc {
    /// Instantiate an `Mdoc`
    pub fn new(
        date: Option<DocumentDate>,
        title: DocumentTitle,
        name: Name,
        description: Description,
        os: Option<OperatingSystem>,
    ) -> Self {
        let mut ret = Self {
            date,
            title,
            os,
            name,
            description,
            ..Default::default()
        };

        ret.lines.push(Line::control(
            "Dd".into(),
            ret.date
                .as_ref()
                .map(|d| vec![d.month.0.clone(), d.day.0.clone(), d.year.0.clone()])
                .unwrap_or_else(|| vec!["$Mdocdate$".into()]),
        ));
        ret.lines.push(Line::control(
            "Dt".into(),
            if let Some(arch) = ret.title.arch.as_ref() {
                vec![
                    ret.title.title.0.clone(),
                    ret.title.section.0.clone(),
                    arch.0.clone(),
                ]
            } else {
                vec![ret.title.title.0.clone(), ret.title.section.0.clone()]
            },
        ));
        ret.lines.push(Line::control("Os".into(), vec![]));
        ret.lines
            .push(Line::control("Sh".into(), vec!["NAME".into()]));
        ret.lines
            .push(Line::control("Nm".into(), vec![ret.name.0.clone()]));
        ret.lines
            .push(Line::control("Nd".into(), vec![ret.description.0.clone()]));

        ret
    }

    pub fn add_section(&mut self, title: impl Into<String>, lines: impl IntoIterator<Item = Line>) {
        self.lines.push(Line::control(
            "Sh".into(),
            vec![title.into().to_uppercase().into()],
        ));
        self.lines.extend(lines)
    }

    /// Append a control line.
    ///
    /// The line consist of the name of a built-in command or macro,
    /// and some number of arguments. Arguments that contain spaces
    /// will be enclosed with double quotation marks.
    pub fn control<'a>(&mut self, name: Str, args: impl IntoIterator<Item = &'a str>) -> &mut Self {
        self.lines.push(Line::control(
            name,
            args.into_iter().map(|s| s.to_string().into()).collect(),
        ));
        self
    }

    /// Append a text line.
    ///
    /// The line will be rendered in a way that ensures it can't be
    /// interpreted as a control line. The caller does not need to
    /// ensure, for example, that the line doesn't start with a
    /// period ("`.`") or an apostrophe ("`'`").
    pub fn text(&mut self, inlines: impl Into<Vec<Inline>>) -> &mut Self {
        self.lines.push(Line::text(inlines.into()));
        self
    }

    /// Render as **mdoc** source text that can be fed to a **mdoc** implementation.
    pub fn render(&self) -> String {
        let mut buf = vec![];
        self.to_writer(&mut buf).unwrap(); // writing to a Vec always works
        String::from_utf8(buf)
            .expect("output is utf8 if all input is utf8 and our API guarantees that")
    }

    /// Write to a writer.
    pub fn to_writer(&self, w: &mut dyn Write) -> Result<(), std::io::Error> {
        for line in self.lines.iter() {
            line.render(w)?;
        }
        Ok(())
    }

    /// Render without handling apostrophes specially.
    ///
    /// You probably want [`render`](Mdoc::render) or
    /// [`to_writer`](Mdoc::to_writer) instead of this method.
    ///
    /// Without special handling, apostrophes get typeset as right
    /// single quotes, including in words like "don't". In most
    /// situations, such as in manual pages, that's unwanted. The
    /// other methods handle apostrophes specially to prevent it, but
    /// for completeness, and for testing, this method is provided to
    /// avoid it.
    pub fn to_mdoc(&self) -> String {
        let mut buf = vec![];
        for line in self.lines.iter() {
            // Writing to a Vec always works, so we discard any error.
            line.render(&mut buf).unwrap();
        }
        String::from_utf8(buf)
            .expect("output is utf8 if all input is utf8 and our API guarantees that")
    }
}

impl<I: Into<Inline>> From<I> for Mdoc {
    fn from(other: I) -> Self {
        let mut r = Mdoc::default();
        r.text([other.into()]);
        r
    }
}

impl<R: Into<Mdoc>> std::iter::FromIterator<R> for Mdoc {
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Self {
        let mut r = Mdoc::default();
        for i in iter {
            r.lines.extend(i.into().lines)
        }
        r
    }
}

impl<R: Into<Mdoc>> Extend<R> for Mdoc {
    fn extend<T: IntoIterator<Item = R>>(&mut self, iter: T) {
        for i in iter {
            self.lines.extend(i.into().lines)
        }
    }
}

/// A part of a text line.
///
/// Text will be escaped for **mdoc**. No inline escape sequences will be
/// passed to **mdoc**. The text may contain newlines, but leading periods
/// will be escaped so that they won't be interpreted by **mdoc** as
/// control lines.
///
/// Note that the strings stored in the variants are stored as they're
/// received from the API user. The Line::render function handles
/// escaping etc.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Inline {
    /// Text in the "roman" font, which is the normal font if nothing
    /// else is specified.
    Roman(String),

    /// Text in the italic (slanted) font.
    Italic(String),

    /// Text in a bold face font.
    Bold(String),

    /// A hard line break. This is an inline element so it's easy to
    /// insert a line break in a paragraph.
    LineBreak,
}

/// Turn a string slice into inline text in the roman font.
///
/// This is equivalent to the [roman] function, but may be more
/// convenient to use.
// impl<S: Into<String>> From<S> for Inline {
//     fn from(s: S) -> Self {
//         roman(s)
//     }
// }

/// Return some inline text in the "roman" font.
///
/// The roman font is the normal font, if no other font is chosen.
pub fn roman(input: impl Into<String>) -> Inline {
    Inline::Roman(input.into())
}

/// Return some inline text in the bold font.
pub fn bold(input: impl Into<String>) -> Inline {
    Inline::Bold(input.into())
}

/// Return some inline text in the italic font.
pub fn italic(input: impl Into<String>) -> Inline {
    Inline::Italic(input.into())
}

/// Return an inline element for a hard line break.
pub fn line_break() -> Inline {
    Inline::LineBreak
}

/// A line in a **mdoc** document.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Line {
    /// A control line.
    Control {
        /// Name of control request or macro being invoked.
        name: Str,

        /// Arguments on control line.
        args: Vec<Str>,
    },

    /// A text line.
    Text(Vec<Inline>),
}

impl Line {
    pub const NAME: Self = Line::control(Cow::Borrowed("Nm"), vec![]);

    /// Append a control line.
    pub const fn control(name: Str, args: Vec<Str>) -> Self {
        Self::Control { name, args }
    }

    /// Append a text line, consisting of inline elements.
    pub fn text(parts: Vec<Inline>) -> Self {
        Self::Text(parts)
    }

    /// Generate a **mdoc** line.
    ///
    /// All the **mdoc** code generation and special handling happens here.
    pub fn render(&self, out: &mut dyn Write) -> Result<(), std::io::Error> {
        match self {
            Self::Control { name, args } => {
                write!(out, ".{}", name)?;
                for arg in args {
                    write!(out, " {}", &arg)?;
                }
            }
            Self::Text(inlines) => {
                let mut at_line_start = true;
                for inline in inlines.iter() {
                    // We need to handle line breaking specially: it
                    // introduces a control line to the **mdoc**, and the
                    // leading period of that mustn't be escaped.
                    match inline {
                        Inline::LineBreak => {
                            if at_line_start {
                                writeln!(out, ".br")?;
                            } else {
                                writeln!(out, "\n.br")?;
                            }
                        }
                        Inline::Roman(text) | Inline::Italic(text) | Inline::Bold(text) => {
                            let text = escape_leading_cc(text);
                            if let Inline::Bold(_) = inline {
                                write!(out, r"\n.Sy {}\n", text)?;
                            } else if let Inline::Italic(_) = inline {
                                write!(out, r"\n.Em {}\n", text)?;
                            } else {
                                if at_line_start && starts_with_period(&text) {
                                    // Line would start with a period, so we
                                    // insert a non-printable, zero-width glyph to
                                    // prevent it from being interpreted as such.
                                    // We only do that when it's needed, though,
                                    // to avoid making the output ugly.
                                    //
                                    // Note that this isn't handled by
                                    // escape_leading_cc, as it
                                    // doesn't know when an inline
                                    // element is at the start of a
                                    // line.
                                    write!(out, r"\&").unwrap();
                                }
                                write!(out, "{}", text)?;
                            }
                        }
                    }
                    at_line_start = false;
                }
            }
        };
        writeln!(out)?;
        Ok(())
    }

    pub fn cross_reference(title: Str, section: Str) -> Self {
        Self::Control {
            name: "Xr".into(),
            args: vec![title, section],
        }
    }
}

/// Does line start with a control character?
#[inline]
pub fn starts_with_period(line: &str) -> bool {
    line.starts_with('.')
}

/// Prevent leading periods or apostrophes on lines to be interpreted
/// as control lines. Note that this needs to be done for apostrophes
/// whether they need special handling for typesetting or not: a
/// leading apostrophe on a line indicates a control line.
pub fn escape_leading_cc(s: &str) -> String {
    s.replace("\n.", "\n\\&.").replace("\n'", "\n\\&'")
}
