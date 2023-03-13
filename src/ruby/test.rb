class Test
  def initialize; end;
  def run
    puts "Hello World!"
  end
end

class Test2 < Test
  def foo
    puts "bar"
  end
end

class AlsoPosition < Position
  def initialize(x,y,z)
    puts "init!"
    super
  end
  def foo()
    puts "bar"
  end
end

Test.new.run
Test2.new.foo
pos = Position.new(1, 2, 3)
puts pos.x
pos.x = 15
puts pos.x
a_pos = AlsoPosition.new(1,2,3)
puts a_pos.y
puts a_pos.class
puts a_pos.foo
