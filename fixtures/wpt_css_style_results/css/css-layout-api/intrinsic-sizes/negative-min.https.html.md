# css/css-layout-api/intrinsic-sizes/negative-min.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/negative-min.https.html"
}
```

## style[0]

```css


.test {
  background: red;
  border: solid 2px;
  height: 100px;
  width: min-content;
}

@supports (display: layout(min-content-size-negative)) {
  .test {
    display: layout(min-content-size-negative);
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
