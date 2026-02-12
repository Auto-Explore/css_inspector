# css/css-layout-api/constraints/fixed-block-size-grid-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-grid-vrl.https.html"
}
```

## style[0]

```css

body {
  display: grid;
  grid: auto-flow / 100px;
}

.test {
  writing-mode: vertical-rl;
  background: red;
  --expected-block-size: 100;
  width: 100px;
}

.child {
  background: green;
}

@supports (display: layout(test)) {
  .test {
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
