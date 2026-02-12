# css/css-lists/marker-counter.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/marker-counter.html"
}
```

## style[0]

```css

ol {
  counter-reset: my-counter;
  padding-left: 5em;
}
::marker {
  counter-increment: my-counter;
  content: "[" counters(my-counter, ":") "] ";
}
.set::marker {
  counter-set: my-counter 10;
}
.reset::marker {
  counter-reset: my-counter;
}
.reset::after {
  counter-increment: my-counter;
  content: " (" counters(my-counter, ":") ")";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
