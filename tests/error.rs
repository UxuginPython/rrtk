use rrtk::error::*;
use rrtk::*;
mod possible_double_error {
    use super::*;
    #[test]
    fn none() {
        let test = PossibleDoubleError::<u8>::from_options(None, None);
        assert!(test.is_none());
    }
    #[test]
    fn a_error() {
        let test = PossibleDoubleError::<u8>::from_options(Some(9), None);
        assert_eq!(test, Some(PossibleDoubleError::A(9)));
    }
    #[test]
    fn b_error() {
        let test = PossibleDoubleError::<u8>::from_options(None, Some(17));
        assert_eq!(test, Some(PossibleDoubleError::B(17)));
    }
    #[test]
    fn ab_error() {
        let test = PossibleDoubleError::<u8>::from_options(Some(83), Some(34));
        assert_eq!(test, Some(PossibleDoubleError::AB(83, 34)));
    }
}
mod nothing_or_error_ext {
    use super::*;
    #[test]
    fn from_option_none() {
        let test = NothingOrError::<u8>::from_option(None);
        assert_eq!(test, Ok(()));
    }
    #[test]
    fn from_option_some() {
        let test = NothingOrError::<u8>::from_option(Some(13));
        assert_eq!(test, Err(13));
    }
    #[test]
    fn into_option_ok() {
        let original: NothingOrError<u8> = Ok(());
        let test = original.into_option();
        assert!(test.is_none());
    }
    #[test]
    fn into_option_err() {
        let original: NothingOrError<u8> = Err(98);
        let test = original.into_option();
        assert_eq!(test, Some(98));
    }
}
