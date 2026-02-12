# css/css-lists/foo-counter-reversed-006e.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/foo-counter-reversed-006e.html"
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
ol[reversed] {
  counter-reset: reversed(foo);
}
li {
  counter-increment: foo -1;
}
ol[reversed] {
  margin-bottom: 0;
}
ol.no-counter {
  counter-reset: blah;
  margin-top: 0;
}
ol.no-counter::before {
  content: none;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
