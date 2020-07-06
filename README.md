# Liquidy

This is a sample demonstrating creating a Ruby module written in Rust which then evaluates Ruby code from inside Rust.

## Setup

You'll need a Ruby installed on your machine and configured in your path, for example with `chruby`:

```sh
chruby 2.6.5
```

At this point you should be able to build the gem:

```sh
rake
```

## Testing it out

There's a script in `bin/console` that will set up a REPL with the gem loaded:

```sh
$ bin/console
irb(main):001:0>
```

At this point you can evaluate some Ruby code through the Rust module:

```
irb(main):004:0> LiquidyVM.evaluate "puts 'Hello world'"
Rust: evaluating "puts \'Hello world\'"
Hello world
Rust: result is Ok(AnyObject { value: Value { value: 8 } })
Rust:    ... as string: Err(#<TypeError: Error converting to String>)
Rust:    ... as number: Err(#<TypeError: Error converting to Fixnum>)
=> nil
```

You'll see that it prints out "Hello world" as expected. It also tries to convert the expression to a string and a number. In this case, `puts` returns `nil` so both those fail. If you give it an
expression then it will print the result there showing that the Rust code can convert the string or numeric values to Rust values:

```
irb(main):001:0> LiquidyVM.evaluate "'hello' + ' ' + 'world'"
Rust: evaluating "\'hello\' + \' \' + \'world\'"
Rust: result is Ok(AnyObject { value: Value { value: 140710980816280 } })
Rust:    ... as string: Ok("hello world")
Rust:    ... as number: Err(#<TypeError: Error converting to Fixnum>)
=> "hello world"
irb(main):006:0> LiquidyVM.evaluate "55 + 42"
Rust: evaluating "55 + 42"
Rust: result is Ok(AnyObject { value: Value { value: 195 } })
Rust:    ... as string: Err(#<TypeError: Error converting to String>)
Rust:    ... as number: Ok(97)
=> 97
```
