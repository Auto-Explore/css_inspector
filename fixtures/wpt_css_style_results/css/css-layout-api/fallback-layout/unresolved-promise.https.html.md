# css/css-layout-api/fallback-layout/unresolved-promise.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fallback-layout/unresolved-promise.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  border: solid 2px;
  width: 100px;
}

.child {
  height: 100px;
}

@supports (display: layout(unresolved-promise)) {
  .test {
    display: layout(unresolved-promise);
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
