
class Circle:
    def __init__(self, r):
        self.radius = r
    def area(self):
        return 3.14*(self.radius*self.radius)

class Square:
    def __init__(self,x):
        self.side = x
    def area(self):
        return self.side * self.side

# is 'a' bigger than 'b'?
def is_bigger(a, b):
    return a.area() > b.area()

# square with side 10
a = Square(10)
b = Circle(10)

# square not bigger than circle
print is_bigger(a, b)


