# SPDX-License-Identifier: Apache-2.0
from memory.types import Point

def create_points(count, default_point=Point()):
    """
    Create a number of points.
    """
    points = []
    for _ in range(count):
        points.append(default_point)
    return points



if __name__ == '__main__':
    # Create a list of points
    sample_points = create_points(5)

    # Modify one point
    sample_points[0].x = 1
    sample_points[0].y = 1

    sample_points[1].x = 2
    sample_points[1].y = 2

    # Check the points
    for point in sample_points:
        print(f"Point(x={point.x}, y={point.y})")

    # Assert
    assert sample_points[0].x == 1 and sample_points[0] == 1, 'The first point was modified!'