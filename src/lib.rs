//! IsFalse
//! 
//! A Rust library for checking if a value is false.

extern crate is_true_rs;
use is_true_rs::IsTrue;

pub trait IsFalse {
    fn is_false(&self) -> bool;
}

impl IsFalse for bool {
    /// Check if the boolean is false
    /// 
    /// # Example
    /// 
    /// ```
    /// extern crate is_false_rs;
    /// use is_false_rs::IsFalse;
    /// 
    /// let x = false;
    /// assert_eq!(x.is_false(), true);
    /// ```
    fn is_false(&self) -> bool {
        !self.is_true()
    }
}

#[cfg(test)]
mod tests {
    use super::IsFalse;

    #[test]
    fn is_false_eq_true() {
        assert_eq!(false.is_false(), true);
    }

    #[test]
    fn is_true_eq_false() {
        assert_eq!(true.is_false(), false);
    }
}
