# css/css-layout-api/intrinsic-sizes/child-size-01.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/child-size-01.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  height: 100px;
  width: min-content;
}

@supports (display: layout(parent)) {
  .test {
    display: layout(parent);
    background: green;
  }

  .child {
    display: layout(child);
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
