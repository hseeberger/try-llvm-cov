#![allow(unused)]
#![cfg_attr(coverage, feature(coverage_attribute))]

fn foo(bar: Option<bool>) -> bool {
    match bar {
        Some(bar) => bar,
        None => false,
    }
}

#[cfg_attr(coverage, coverage(off))]
fn baz(bar: Option<bool>) -> bool {
    match bar {
        Some(bar) => bar,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn test_foo() {
        assert!(!foo(None));
        assert!(foo(Some(true)));
    }
}
