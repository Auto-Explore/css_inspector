# css/css-sizing/min-max-content-orthogonal-flow-crash-001.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/min-max-content-orthogonal-flow-crash-001.html"
}
```

## style[0]

```css

body {
  width: fit-content;
}
#target {
  display: block;
  writing-mode: vertical-lr;
  columns: 2;
}
.after #target {
  float: left;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
