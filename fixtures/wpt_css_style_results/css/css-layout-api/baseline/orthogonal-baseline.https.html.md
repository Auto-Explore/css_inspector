# css/css-layout-api/baseline/orthogonal-baseline.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/baseline/orthogonal-baseline.https.html"
}
```

## style[0]

```css

.test {
  background: green;
  padding: 0 10px;
  width: 80px;
}

@supports (display: layout(test)) {
  .test {
    display: layout(test);
    background: green;
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
