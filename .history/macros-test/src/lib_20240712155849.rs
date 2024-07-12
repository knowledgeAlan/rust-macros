mod auto_debug;
mod auto_deref;
mod enum_from_darling;
mod enum_from;

use auto_debug::process_auto_debug;
use auto_deref::process_auto_deref;
use enum_from::process_enum_from;
use enum_from_darling::process_enum_from_darling;
use proc_macro::TokenStream;
use sync::DeriveInput;


