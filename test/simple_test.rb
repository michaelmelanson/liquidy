require "test_helper"

class SimpleTest < Minitest::Test
  def test_text_returns_same_text
    assert_equal "hello world", Liquidy.render_string("hello world")
  end

  def test_substitution
    assert_equal "hi tobi!", Liquidy.render_string("hi {{name}}!", "name": "tobi")
  end
end
