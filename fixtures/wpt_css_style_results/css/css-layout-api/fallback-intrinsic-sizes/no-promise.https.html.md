# css/css-layout-api/fallback-intrinsic-sizes/no-promise.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fallback-intrinsic-sizes/no-promise.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  border: solid 2px;
  width: min-content;
}

.child {
  height: 100px;
}

@supports (display: layout(no-promise)) {
  .test {
    display: layout(no-promise);
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
