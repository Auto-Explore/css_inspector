# css/css-layout-api/fallback-intrinsic-sizes/constructor-error.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/fallback-intrinsic-sizes/constructor-error.https.html"
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
  width: 50%;
  height: 100px;
}

.fc {
  display: flow-root;
  height: 100px;
}

@supports (display: layout(throwing-ctor)) {
  .test {
    display: layout(throwing-ctor);
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
