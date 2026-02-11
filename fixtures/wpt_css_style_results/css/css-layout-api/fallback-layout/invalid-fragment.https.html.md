# css/css-layout-api/fallback-layout/invalid-fragment.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fallback-layout/invalid-fragment.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  border: solid 2px;
  width: 100px;
}

.test > div {
  height: 100px;
}

@supports (display: layout(bad-request)) {
  .test {
    display: layout(bad-request);
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
