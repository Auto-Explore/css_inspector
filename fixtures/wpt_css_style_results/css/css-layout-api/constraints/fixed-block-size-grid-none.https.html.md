# css/css-layout-api/constraints/fixed-block-size-grid-none.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-grid-none.https.html"
}
```

## style[0]

```css

body {
  display: grid;
}

.test {
  background: red;
  --expected-block-size: null;
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
