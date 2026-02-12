# css/css-lists/foo-counter-reversed-009b.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/foo-counter-reversed-009b.html"
}
```

## style[0]

```css

:root {
  color:black; background-color:white; font:10px/1 monospace;
}
::marker {
  content: counter(foo) ". ";
}
ol::before, li::before {
  content: counters(foo,".");
}
div, span {
  counter-increment: foo 0;
}
.pure-set {
  counter-increment: foo 0;
  counter-set: foo 5;
}
ol[reversed] {
  counter-reset: reversed(foo);
}
li {
  counter-increment: foo -1 list-item 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
