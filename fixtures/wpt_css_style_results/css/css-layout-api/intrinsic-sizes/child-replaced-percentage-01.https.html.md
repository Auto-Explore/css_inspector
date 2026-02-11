# css/css-layout-api/intrinsic-sizes/child-replaced-percentage-01.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/child-replaced-percentage-01.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  height: 50px;
  width: min-content;
}

@supports (display: layout(parent)) {
  .test {
    display: layout(parent);
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
