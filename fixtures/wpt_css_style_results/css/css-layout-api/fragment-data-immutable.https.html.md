# css/css-layout-api/fragment-data-immutable.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fragment-data-immutable.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  width: 100px;
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
