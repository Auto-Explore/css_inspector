# css/css-layout-api/constraints/fixed-block-size-block-none-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-block-none-vrl.https.html"
}
```

## style[0]

```css

.test {
  writing-mode: vertical-lr;
  background: red;
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
