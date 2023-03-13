use super::primitives::*;

#[test]
fn test_trio() {
    assert_eq!(
        Point::from([1.0, 2.0, 3.0]) + Point::from([2.0, 3.0, 4.0]),
        Point::from([3.0, 5.0, 7.0])
    );
    assert_eq!(
        Point::from([1.0, -2.5, 3.8]) + Point::from([2.03, -3.44, -4.0]),
        Point::from([3.03, -5.94, -0.2])
    );
    assert_eq!(
        Point::from([0.0, 0.0, 0.001]) + Point::from([2.03, 0.0, -4.0]),
        Point::from([2.03, 0.0, -3.999])
    );

    assert_eq!(
        Point::from([3.0, 5.0, 7.0]) - Point::from([2.0, 3.0, 4.0]),
        Point::from([1.0, 2.0, 3.0])
    );
    assert_eq!(
        Point::from([3.0, 5.0, 7.0]) - Point::from([1.0, 2.0, 3.0]),
        Point::from([2.0, 3.0, 4.0])
    );
    assert_eq!(
        Point::from([3.03, -5.94, -0.2]) - Point::from([2.03, -3.44, -4.0]),
        Point::from([1.0, -2.5, 3.8])
    );
    assert_eq!(
        Point::from([3.03, -5.94, -0.2]) - Point::from([1.0, -2.5, 3.8]),
        Point::from([2.03, -3.44, -4.0])
    );

    assert_eq!(
        Point::from([1.0, -2.5, 3.8]) * 0.0,
        Point::default()
    );
    assert_eq!(
        Point::from([3.0, 5.0, 7.0]) * 1.23,
        Point::from([3.69, 6.15, 8.61])
    );

    assert_eq!(
        Point::from([3.0, 5.0, 7.0]) / 1.23,
        Point::from([3.0 / 1.23, 5.0 / 1.23, 7.0 / 1.23])
    );
}