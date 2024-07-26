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

use std::borrow::Cow;

use super::*;

impl From<::clap::Command> for Mdoc {
    fn from(cmd: ::clap::Command) -> Self {
        let mut m = Mdoc::new(
            None,
            DocumentTitle {
                title: title! {cmd.get_display_name().unwrap_or_else(|| cmd.get_name()).to_string() },
                section: section! { "1" },
                arch: None,
            },
            name! { cmd.get_bin_name().unwrap_or_else(|| cmd.get_name()).to_string() },
            description! { cmd.get_about().unwrap_or_default().to_string() },
            None,
        );
        m.control("Sh".into(), vec!["SYNOPSIS"]);
        m.control("Nm".into(), vec![]);
        for opt in cmd.get_opts() {
            let mut v: Vec<Cow<'_, str>> = vec![];
            let required = opt.is_required_set();
            let control = if required {
                "Fl".into()
            } else {
                v.push("Fl".into());
                "Op".into()
            };
            if let Some(long) = opt.get_long() {
                let s = format!("-{long}");
                v.push(Cow::Owned(s));
                if let Some(short) = opt.get_short() {
                    let s = format!("{short}");
                    v.push(Cow::Borrowed("|"));
                    v.push(Cow::Owned(s));
                }
            } else if let Some(short) = opt.get_short() {
                let s = format!("{short}");
                v.push(Cow::Owned(s));
            } else {
                continue;
            }

            match opt.get_action() {
                clap::ArgAction::Set => {
                    v.push(Cow::Borrowed("Ar"));
                    if let Some(val) = opt.get_value_names().unwrap_or_default().first() {
                        v.push(Cow::Borrowed(val.as_str()));
                    } else {
                        v.push(Cow::Borrowed("VALUE"));
                    }
                }
                clap::ArgAction::Append => {}
                clap::ArgAction::SetTrue | clap::ArgAction::SetFalse => {}
                clap::ArgAction::Count => {}
                _ => {}
            }
            m.control(control, v.iter().map(|c| c.as_ref()));
        }
        for _opt in cmd.get_positionals() {}
        m.control("Sh".into(), vec!["DESCRIPTION"]);
        if let Some(author) = cmd.get_author() {
            // .An Name Aq Mt user@example.com
            m.control("Sh".into(), vec!["AUTHORS"]);
            m.control("An".into(), author.split(' ').collect::<Vec<&str>>());
        }
        m
    }
}
