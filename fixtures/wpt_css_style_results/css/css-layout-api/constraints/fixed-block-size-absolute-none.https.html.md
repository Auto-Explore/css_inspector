# css/css-layout-api/constraints/fixed-block-size-absolute-none.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-absolute-none.https.html"
}
```

## style[0]

```css

body {
  position: relative;
}

.test {
  background: red;
  position: absolute;
  --expected-block-size: null;
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
