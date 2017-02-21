
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

# square with side 10
a = Square(10)
print a.area()
