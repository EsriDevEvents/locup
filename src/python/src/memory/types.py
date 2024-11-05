# SPDX-License-Identifier: Apache-2.0
class Point:
    """Simple point structure"""

    def __init__(self, x=0, y=0, wkid=4326):
        """
        Creates a new point instance.
        """
        self.x = x
        self.y = y
        self.wkid = wkid