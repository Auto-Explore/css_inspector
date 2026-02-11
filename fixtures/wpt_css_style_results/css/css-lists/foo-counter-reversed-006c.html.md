# css/css-lists/foo-counter-reversed-006c.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/foo-counter-reversed-006c.html"
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
  counter-reset: reversed(foo) reversed(list-item);
}
li {
  counter-increment: foo -1 list-item -1;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “counter-reset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
