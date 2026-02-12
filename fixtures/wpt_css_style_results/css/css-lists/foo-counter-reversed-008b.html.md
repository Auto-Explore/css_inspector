# css/css-lists/foo-counter-reversed-008b.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/foo-counter-reversed-008b.html"
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
ol[reversed] {
  counter-reset: reversed(list-item) 999 reversed(foo);
}
li {
  counter-increment: list-item 999 foo -1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
