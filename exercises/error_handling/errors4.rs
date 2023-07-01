// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if (value == 0) {
            return Err(CreationError::Zero);
        }
        if (value < 0) {
            return Err(CreationError::Negative);
        }
        // Hmm...? Why is this only returning an Ok value?
        Ok(PositiveNonzeroInteger(value as u64))

        /*
        wooooowwwww i was doing the most. finally used the `hint` watch subcommand,
        solved in no time
        */

        // let eek = PositiveNonzeroInteger(value as u64);
        // let eeeep = match eek {
        //     // let why = match value {
        //     // Ok(x) => Ok(PositiveNonzeroInteger(x as u64)),
        //     Ok(x) => Ok(PositiveNonzeroInteger(x as u64)),
        //     Err(x) => match x.kind() {
        //         CreationError::Negative => Err(CreationError::Negative),
        //         CreationError::Zero => Err(CreationError::Zero),
        //     }, // {
        //        //     if x == 0 {
        //        //         return Err(CreationError::Zero);
        //        //     }
        //        // }
        //        // Err(x) => {
        //        //     if x < 0 {
        //        //         return Err(CreationError::Negative);
        //        //     }
        //        // }
        // };
        // return eeeep;
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
