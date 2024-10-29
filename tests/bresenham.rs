use stara::bresenham::bresenham_line;

#[test]
fn test_horizontal_line() {
    let start = (2, 5);
    let end = (7, 5);
    let expected_points = vec![(2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5)];
    assert_eq!(
        bresenham_line(start.0, start.1, end.0, end.1),
        expected_points
    );
}

#[test]
fn test_vertical_line() {
    let start = (4, 2);
    let end = (4, 6);
    let expected_points = vec![(4, 2), (4, 3), (4, 4), (4, 5), (4, 6)];
    assert_eq!(
        bresenham_line(start.0, start.1, end.0, end.1),
        expected_points
    );
}

#[test]
fn test_diagonal_line() {
    let start = (1, 1);
    let end = (4, 4);
    let expected_points = vec![(1, 1), (2, 2), (3, 3), (4, 4)];
    assert_eq!(
        bresenham_line(start.0, start.1, end.0, end.1),
        expected_points
    );
}

#[test]
fn test_steep_slope_line() {
    let start = (2, 2);
    let end = (5, 10);
    let expected_points = vec![
        (2, 2),
        (2, 3),
        (3, 4),
        (3, 5),
        (3, 6),
        (4, 7),
        (4, 8),
        (5, 9),
        (5, 10),
    ];
    assert_eq!(
        bresenham_line(start.0, start.1, end.0, end.1),
        expected_points
    );
}

#[test]
fn test_shallow_slope_line() {
    let start = (2, 2);
    let end = (8, 4);
    let expected_points = vec![(2, 2), (3, 2), (4, 3), (5, 3), (6, 3), (7, 4), (8, 4)];
    assert_eq!(
        bresenham_line(start.0, start.1, end.0, end.1),
        expected_points
    );
}

#[test]
fn test_negative_slope_line() {
    let start = (6, 8);
    let end = (3, 5);
    let expected_points = vec![(6, 8), (5, 7), (4, 6), (3, 5)];
    assert_eq!(
        bresenham_line(start.0, start.1, end.0, end.1),
        expected_points
    );
}

#[test]
fn test_reverse_line() {
    let start = (5, 5);
    let end = (2, 2);
    let expected_points = vec![(5, 5), (4, 4), (3, 3), (2, 2)];
    assert_eq!(
        bresenham_line(start.0, start.1, end.0, end.1),
        expected_points
    );
}
