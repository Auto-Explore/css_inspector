# css/css-layout-api/constraints/fixed-block-size-fixed.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-fixed.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  --expected-block-size: 60;
  width: 100px;
  height: 60px;
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
