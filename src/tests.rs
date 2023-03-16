use crate::VECTOR_SPACE;
use super::primitives::*;

#[test]
fn test_point_vector_ops() {
    assert_eq!(
        Point::from((1, -2.5, 3.8)) + Point::from((2.03, -3.44, -4)),
        Point::from((3.03, -5.94, -0.2))
    );
    assert_eq!(
        Vector::from((3.03, -5.94, -0.2)) - Vector::from((2.03, -3.44, -4)),
        Vector::from((1, -2.5, 3.8))
    );
    assert_eq!(
        Point::from((3, 5, 7)) * 1.23,
        Point::from((3.69, 6.15, 8.61))
    );
    assert_eq!(
        -3.56 * Point::from((4.2, 0.3, -7.09)),
        Point::from((-14.952, -1.068, 25.2404))
    );
    assert_eq!(
        Vector::from((3, 5, 7)) / 1.23,
        Vector::from((3.0 / 1.23, 5.0 / 1.23, 7.0 / 1.23))
    );

    assert_eq!(Vector::from((3, 0, 7)) * Vector::from((0, 2, 0)), 0.0);
    assert_eq!(Vector::from((3, 5, -7)) * Vector::from((11, -13.0, 17)), -151.0);

    assert_eq!(
        Vector::from((1, 0, 3)) ^ Vector::from((-2, 0, -6)),
        Vector::default()
    );
    assert_eq!(
        Vector::from((1, 0, 3)) ^ Vector::default(),
        Vector::default()
    );
    assert_eq!(
        Vector::from((2, 3, 5)) ^ Vector::from((7, 11, 13)),
        Vector::from((-16, 9, 1))
    );
    assert_eq!(
        Vector::from((7, 11, 13)) ^ Vector::from((2, 3, 5)),
        Vector::from((16, -9, -1))
    );
    assert_eq!(
        Vector::from((3.03, -5.94, -0.2)) ^ Vector::from((2.03, -3.44, -4)),
        Vector::from((
            -5.94 * -4.0 - -0.2 * -3.44,
            -0.2 * 2.03 - 3.03 * -4.0,
            3.03 * -3.44 - -5.94 * 2.03
        ))
    );
}