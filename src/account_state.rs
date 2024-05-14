//! State based implementation of bank account "objects"
//! 
//! TODO: define signatures and other aspects of spec
//! TODO: generate tests for behavior
//! TODO: implement/code

pub mod account_state {

    /// Trait describing shared behavior for both account types with default methods
    /// and will contain all the method signatures
    pub trait AccountBehavior {}

    /// Struct describing a bank account in good standing
    pub struct ValidAccount {}

    /// Struct describing a bank account in bad standing
    pub struct InvalidAccount {}
}

/// Unit tests for the module
#[cfg(test)]
mod tests {use super::*;}