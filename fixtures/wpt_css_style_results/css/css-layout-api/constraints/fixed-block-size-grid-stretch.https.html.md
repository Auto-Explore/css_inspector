# css/css-layout-api/constraints/fixed-block-size-grid-stretch.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-grid-stretch.https.html"
}
```

## style[0]

```css

body {
  display: grid;
  grid: 50px / auto-flow;
}

.test {
  background: red;
  --expected-block-size: 40; /* grid-item should stretch to (50 - 10)px */
  width: 100px;
  margin-bottom: 10px;
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
