# Conway
This is a small side project I've been working on. Perhaps my third or fourth iteration of creating a PL and to teach myself type theory in the process. The project is in early stages and it might tend to break a lot.

If you are familiar with Scala, Ruby or Rust, you might find in Conway some similitudes. I like how such languages feels so natural so I decided to take some inspiration from them. However, currently some designs decisions are being made, hence the grammar is up to changes as long as I add more features and fix some inconsistencies. Switch to the 'functions' branch for the latest changes.

# Examples
Conway can evaluate functions such as:
```
fn foo(): String do
  "Hi hello!"
end

fn baz(b): Bool do
  !b
end

```
Use the 'let' keyword to declare a new variable (Nil, by default).
```
=> let bar = true
=> print bar
=> true
```
Re assign its value.
```
=> bar = 'String literal here'
=> print bar
=> 'String literal here'
```
Dynamic variable scoping.
```
let a = 'Kaboom!'
{
  print a -> 'Kaboom!'
}

{
  let a = 'Oh hi Mark'
}
print a -> This variable is undeclared!

```
And perform some operations.
```
=> !true
=> false
```

# Roadmap
- [x] Basic primitive types.
- [x] Unary operations.
- [ ] Binary operations.
- [x] Variable declaration and re assignment.
- [x] Variable scope.
- [x] Code blocks.
- [x] Functions.
- [ ] Design custom error types for parsing stage.
- [ ] If-else statements.
- [ ] Vectors.
- [ ] Let expressions.
- [ ] Closures.
- [ ] A stable REPL.
- [ ] Meaningful error messages at runtime.
- [ ] Mutable and non-mutable variables.
- [ ] FP features.
- [ ] Type checking

# Why Conway?
Because of Conway The Machine, the rapper :).
