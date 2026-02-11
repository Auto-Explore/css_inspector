# css/css-layout-api/fallback-layout/invalid-child.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fallback-layout/invalid-child.https.html"
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

@supports (display: layout(bad-child-layout)) {
  .test {
    display: layout(bad-child-layout);
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
