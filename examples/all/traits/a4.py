
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

class Dog:
    pass

# is 'a' bigger than 'b'?
def is_bigger(a, b):
    return a.area() > b.area()

a = Square(10)
b = Dog()

# comparing a square and a dog!
print is_bigger(a, b)


