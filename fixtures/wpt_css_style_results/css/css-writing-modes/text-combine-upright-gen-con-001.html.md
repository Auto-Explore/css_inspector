# css/css-writing-modes/text-combine-upright-gen-con-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-combine-upright-gen-con-001.html"
}
```

## style[0]

```css

div {
  writing-mode: vertical-rl;
  font: 32px serif;
  margin: 1em;
  padding: .25em;
  border: 1px solid gray;
}
div#test {
  counter-set: a 3 b 1;
}
p#a::before {
  text-combine-upright: all;
  content: '1';
}
p#b::before {
  text-combine-upright: all;
  content: '(' '2' ')';
}
p#c::before {
  text-combine-upright: all;
  content: '(' counter(a, decimal) '.' counter(b, lower-alpha) ')';
}
p#d::after {
  text-combine-upright: all;
  content: 'h' 'e' 'll' 'o';
}
span {
  text-combine-upright: all;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
