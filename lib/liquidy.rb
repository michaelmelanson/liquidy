require 'liquidy/version'
require 'rutie'

module Liquidy
  Rutie.new(:liquidy).init 'init_liquidy', __dir__

  def self.render_string(liquid, **context)
    template = Liquidy.parse(liquid)
    Liquidy.render(template, **context)
  end
end
