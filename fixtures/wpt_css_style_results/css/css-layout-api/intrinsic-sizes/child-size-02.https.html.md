# css/css-layout-api/intrinsic-sizes/child-size-02.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/child-size-02.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  height: 100px;
  width: min-content;
}

.child {
  min-width: 75px;
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
