# css/css-layout-api/intrinsic-sizes/negative-max.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/negative-max.https.html"
}
```

## style[0]

```css


.test {
  background: red;
  border: solid 2px;
  height: 100px;
  width: max-content;
}

@supports (display: layout(max-content-size-negative)) {
  .test {
    display: layout(max-content-size-negative);
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
