use std::iter;

use crate::util::eq_ignore_case;

/// The representation of a possible value of an argument.
///
/// This is used for specifying [possible values] of [Args].
///
/// **NOTE:** This struct is likely not needed for most usecases as it is only required to
/// [hide] single values from help messages and shell completions or to attach [about] to possible values.
///
/// # Examples
///
/// ```rust
/// # use clap::{Arg, PossibleValue};
/// let cfg = Arg::new("config")
///       .takes_value(true)
///       .value_name("FILE")
///       .possible_value(PossibleValue::new("fast"))
///       .possible_value(PossibleValue::new("slow").about("slower than fast"))
///       .possible_value(PossibleValue::new("secret speed").hidden(true));
/// ```
/// [Args]: crate::Arg
/// [possible values]: crate::Arg::possible_value()
/// [hide]: PossibleValue::hidden()
/// [about]: PossibleValue::about()
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PossibleValue<'help> {
    pub(crate) name: &'help str,
    pub(crate) about: Option<&'help str>,
    pub(crate) aliases: Vec<&'help str>, // (name, visible)
    pub(crate) hidden: bool,
}

impl<'help> From<&'help str> for PossibleValue<'help> {
    fn from(s: &'help str) -> Self {
        Self::new(s)
    }
}

impl<'help> From<&'help &'help str> for PossibleValue<'help> {
    fn from(s: &'help &'help str) -> Self {
        Self::new(s)
    }
}

/// Getters
impl<'help> PossibleValue<'help> {
    /// Get the name of the argument value
    #[inline]
    pub fn get_name(&self) -> &str {
        self.name
    }

    /// Get the help specified for this argument, if any
    #[inline]
    pub fn get_about(&self) -> Option<&str> {
        self.about
    }

    /// Should the value be hidden from help messages and completion
    #[inline]
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Get the name if argument value is not hidden, `None` otherwise
    pub fn get_visible_name(&self) -> Option<&str> {
        if self.hidden {
            None
        } else {
            Some(self.name)
        }
    }

    /// Returns all valid values of the argument value.
    /// Namely the name and all aliases.
    pub fn get_name_and_aliases(&self) -> impl Iterator<Item = &str> {
        iter::once(&self.name).chain(&self.aliases).copied()
    }

    /// Tests if the value is valid for this argument value
    ///
    /// The value is valid if it is either the name or one of the aliases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::PossibleValue;
    /// let arg_value = PossibleValue::new("fast").alias("not-slow");
    ///
    /// assert!(arg_value.matches("fast", false));
    /// assert!(arg_value.matches("not-slow", false));
    ///
    /// assert!(arg_value.matches("FAST", true));
    /// assert!(!arg_value.matches("FAST", false));
    /// ```
    pub fn matches(&self, value: &str, ignore_case: bool) -> bool {
        if ignore_case {
            self.get_name_and_aliases()
                .any(|name| eq_ignore_case(name, value))
        } else {
            self.get_name_and_aliases().any(|name| name == value)
        }
    }
}

impl<'help> PossibleValue<'help> {
    /// Creates a new instance of [`PossibleValue`] using a string name. The name will be used to
    /// decide wether this value was provided by the user to an argument.
    ///
    /// **NOTE:** In case it is not [hidden] it will also be shown in help messages for arguments
    /// that use it as a [possible value] and have not hidden them through [`Arg::hide_possible_values(true)`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::PossibleValue;
    /// PossibleValue::new("fast")
    /// # ;
    /// ```
    /// [hidden]: PossibleValue::hidden
    /// [possible value]: crate::Arg::possible_values
    /// [`Arg::hide_possible_values(true)`]: crate::Arg::hide_possible_values()
    pub fn new(name: &'help str) -> Self {
        PossibleValue {
            name,
            ..Default::default()
        }
    }

    /// Sets the help text of the value that will be displayed to the user when completing the
    /// value in a compatible shell. Typically, this is a short description of the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::PossibleValue;
    /// PossibleValue::new("slow")
    ///     .about("not fast")
    /// # ;
    /// ```
    #[inline]
    pub fn about(mut self, about: &'help str) -> Self {
        self.about = Some(about);
        self
    }

    /// Hides this value from help text and shell completions.
    ///
    /// This is an alternative to hiding through [`Arg::hide_possible_values(true)`], if you only
    /// want to hide some values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::PossibleValue;
    /// PossibleValue::new("secret")
    ///     .hidden(true)
    /// # ;
    /// ```
    /// [`Arg::hide_possible_values(true)`]: crate::Arg::hide_possible_values()
    #[inline]
    pub fn hidden(mut self, yes: bool) -> Self {
        self.hidden = yes;
        self
    }

    /// Sets an alias for this argument value.
    ///
    /// The alias will be hidden from completion and help texts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::PossibleValue;
    /// PossibleValue::new("slow")
    ///     .alias("not-fast")
    /// # ;
    /// ```
    pub fn alias(mut self, name: &'help str) -> Self {
        self.aliases.push(name);
        self
    }

    /// Sets multiple aliases for this argument value.
    ///
    /// The aliases will be hidden from completion and help texts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::PossibleValue;
    /// PossibleValue::new("slow")
    ///     .aliases(["not-fast", "snake-like"])
    /// # ;
    /// ```
    pub fn aliases<I>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = &'help str>,
    {
        self.aliases.extend(names.into_iter());
        self
    }
}
