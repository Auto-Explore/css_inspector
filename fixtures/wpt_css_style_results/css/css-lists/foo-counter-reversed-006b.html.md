# css/css-lists/foo-counter-reversed-006b.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/foo-counter-reversed-006b.html"
}
```

## style[0]

```css

:root {
  color:black; background-color:white; font:10px/1 monospace;
}
ol {
  counter-reset: reversed(foo);
}
::marker {
  content: counter(foo) ". ";
}
ol::before, li::before {
  content: counters(foo,".");
}
li {
  counter-increment: foo -1;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “counter-reset”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
