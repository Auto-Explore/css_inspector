# css/css-layout-api/fallback-intrinsic-sizes/bad-return.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fallback-intrinsic-sizes/bad-return.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  border: solid 2px;
  width: min-content;
}

.float {
  float: left;
  height: 100px;
  width: 50%;
}

.fc {
  display: flow-root;
  height: 100px;
}

@supports (display: layout(bad-return)) {
  .test {
    display: layout(bad-return);
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
