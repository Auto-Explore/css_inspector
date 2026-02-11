# css/css-layout-api/intrinsic-sizes/content-size.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/content-size.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  border: solid 2px;
  height: 100px;
}

.test-1 {
  width: max-content;
}

.test-2 {
  width: min-content;
}

@supports (display: layout(test-layout)) {
  .test {
    display: layout(test-layout);
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
