require "test/unit"
require "demo_module"
 
class TestAdd < Test::Unit::TestCase
  def test_add
    assert_equal(5, DemoModule.add(2, 3) )
  end
end