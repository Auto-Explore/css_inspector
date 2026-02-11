# css/css-layout-api/fragment-data-sab-failure.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fragment-data-sab-failure.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  width: 100px;
}

.child {
  height: 100px;
}

@supports (display: layout(fallback-sab)) {
  .test {
    display: layout(fallback-sab);
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
