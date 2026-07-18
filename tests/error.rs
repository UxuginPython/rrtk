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
