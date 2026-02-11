# css/css-layout-api/baseline/child-baseline.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/baseline/child-baseline.https.html"
}
```

## style[0]

```css

.test {
  background: green;
  padding: 0 10px;
  width: 80px;
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
