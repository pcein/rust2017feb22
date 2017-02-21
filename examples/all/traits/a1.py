
class Circle:
    def __init__(self, r):
        self.radius = r
    def area(self):
        return 3.14*(self.radius*self.radius)

# circle with radius 10
a = Circle(10) 
print a.area()
