use std::convert::From;

use binding::rproc;
use types::{Value, Callback};
use util;

use {AnyObject, Class, Object, VerifiedObject, NilClass};

/// `Proc` (works with `Lambda` as well)
#[derive(Debug, PartialEq)]
pub struct Proc {
    value: Value,
}

impl Proc {
    /// Creates a new Ruby proc from a closure
    ///
    /// # Examples
    ///
    /// ```
    /// use rutie::{Class, Object, Proc, RString};
    ///
    /// let closure = || {
    ///     let word = RString::new("Rust");
    ///     word.send("reverse", None)
    /// };
    ///
    /// let proc = Proc::new(closure, None);
    /// let result = proc.call(None);
    ///
    /// match result.try_convert_to::<RString>() {
    ///     Ok(val) => assert_eq!("tsuR", val.to_str()),
    ///     Err(_) => unreachable!(),
    /// }
    /// ```
    pub fn new<F, R>(closure: F, binding: Option<AnyObject>) -> Self
      where F: 'static + FnOnce() -> R, R: Object {

      let bind = match binding {
          Some(item) => item,
          _ => NilClass::new().to_any_object(),
      }.value();

      Proc { value: rproc::new(closure, bind) }
    }

    /// Calls a proc with given arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate rutie;
    ///
    /// use rutie::{Class, Object, Proc, RString};
    ///
    /// class!(Greeter);
    ///
    /// methods!(
    ///     Greeter,
    ///     itself,
    ///
    ///     fn greet_rust_with(greeting_template: Proc) -> RString {
    ///         let name = RString::new("Rust").to_any_object();
    ///         let rendered_template = greeting_template.unwrap().call(Some(&[name]));
    ///
    ///         rendered_template.try_convert_to::<RString>().unwrap()
    ///     }
    /// );
    ///
    /// fn main() {
    ///     Class::new("Greeter", None).define(|itself| {
    ///         itself.def_self("greet_rust_with", greet_rust_with);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Greeter
    ///   def self.greet_rust_with(greeting_template)
    ///     greeting_template.call('Rust')
    ///   end
    /// end
    ///
    /// greeting_template = -> (name) { "Hello, #{name}!" }
    ///
    /// Greeter.greet_rust_with(greeting_template) # => "Hello, Rust!"
    /// ```
    pub fn call(&self, arguments: Option<&[AnyObject]>) -> AnyObject {
        let arguments = util::arguments_to_values(arguments).unwrap_or_default();
        let result = rproc::call(self.value(), &arguments);

        AnyObject::from(result)
    }
}

impl From<Value> for Proc {
    fn from(value: Value) -> Self {
        Proc { value: value }
    }
}

impl Object for Proc {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Proc {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Class::from_existing("Proc")
    }

    fn error_message() -> &'static str {
        "Error converting to Proc"
    }
}
