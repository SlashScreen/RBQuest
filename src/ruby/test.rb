class Test
  def initialize; end;
  def run
    puts "Hello World!"
  end
end

class AlsoPosition < Position
  def foo()
    puts "bar"
  end
end

Test.new.run
pos = Position.new(1, 2, 3)
puts pos.x
pos.set_x(15)
puts pos.x
a_pos = AlsoPosition.new(1,2,3)
puts a_pos.y
#puts a_pos.foo

