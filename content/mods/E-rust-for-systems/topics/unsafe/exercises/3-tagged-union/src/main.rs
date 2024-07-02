use std::mem::ManuallyDrop;

// An implementation of the `Result` type using explicitly tagged unions
//
// fix the TODOs, and verify that your implementation mirrors the one of `std::result::Result`
// using tests.
//
// > cargo test -p F3-tagged-union

/// RocResult matches the memory layout of a result type generated by the roc compiler. By
/// mirroring the layout on the rust side, values can be passed between roc and rust without any
/// (de)serialization cost
struct RocResult<T, E> {
    tag: RocResultTag,
    payload: RocResultUnion<T, E>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RocResultTag {
    Ok,
    Err,
}

// HINT: read the documentation of ManuallyDrop, and see what methods it provides
union RocResultUnion<T, E> {
    ok: ManuallyDrop<T>,
    err: ManuallyDrop<E>,
}

impl<T, E> Drop for RocResult<T, E> {
    fn drop(&mut self) {
        // implement drop. Make sure values wrapped in a ManuallyDrop are dropped correctly!
        todo!()
    }
}

impl<T, E> Clone for RocResult<T, E> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T, E> RocResult<T, E> {
    fn unwrap(mut self) -> T {
        match self.tag {
            RocResultTag::Ok => unsafe { ManuallyDrop::take(&mut self.payload.ok) },
            RocResultTag::Err => panic!("Called `unwrap` on an Err"),
        }
    }

    fn unwrap_err(mut self) -> E {
        todo!()
    }

    fn ok(v: T) -> Self {
        todo!()
    }

    fn err(e: E) -> Self {
        todo!()
    }

    fn is_ok(&self) -> bool {
        matches!(self.tag, RocResultTag::Ok)
    }

    fn is_err(&self) -> bool {
        todo!()
    }

    fn map<F, U>(mut self, f: F) -> RocResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        todo!()
    }
}

impl<T, E> From<RocResult<T, E>> for Result<T, E> {
    fn from(value: RocResult<T, E>) -> Self {
        todo!()
    }
}

impl<T, E> From<Result<T, E>> for RocResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ok() {
        let ok: RocResult<i32, String> = RocResult::ok(42);

        assert!(ok.is_ok());
        assert!(Result::from(ok).is_ok());
    }

    #[test]
    fn test_is_err() {
        let err: RocResult<(), i32> = RocResult::err(42);

        assert!(err.is_err());
        assert!(Result::from(err).is_err());
    }

    #[test]
    fn test_map() {
        let ok: RocResult<i32, i128> = RocResult::ok(42);

        let ok = ok.map(|n| format!("{n}"));

        assert_eq!(ok.unwrap(), "42");
    }

    #[test]
    fn test_clone_ok() {
        let ok: RocResult<i32, i128> = RocResult::ok(42);

        let ok = ok.map(|n| format!("{n}"));

        let std_ok = Result::from(ok.clone());

        assert_eq!(ok.unwrap(), std_ok.unwrap());
    }

    #[test]
    fn test_clone_err() {
        let err: RocResult<i32, String> = RocResult::err(String::from("Hello World!"));

        let std_err = Result::from(err.clone());

        assert_eq!(err.unwrap_err(), std_err.unwrap_err());
    }
}

fn main() {
    println!("Hello, world!");
}
