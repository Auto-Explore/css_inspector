# css/css-layout-api/constraints/fixed-inline-size-grid.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-grid.https.html"
}
```

## style[0]

```css

body {
  display: grid;
  grid: auto-flow / 100px;
}

.test {
  background: red;
}

@supports (display: layout(test)) {
  .test {
    background: green;
    display: layout(test);
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
