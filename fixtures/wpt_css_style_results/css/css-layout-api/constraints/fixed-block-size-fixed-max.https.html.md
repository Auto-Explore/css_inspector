# css/css-layout-api/constraints/fixed-block-size-fixed-max.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-fixed-max.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  --expected-block-size: 30;
  width: 100px;
  height: 60px;
  max-height: 30px;
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
