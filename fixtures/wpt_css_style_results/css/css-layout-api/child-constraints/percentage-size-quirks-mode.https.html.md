# css/css-layout-api/child-constraints/percentage-size-quirks-mode.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/percentage-size-quirks-mode.https.html"
}
```

## style[0]

```css

.container {
  height: 200px;
}

.test {
  background: red;
  width: 100px;
}

.child {
  visibility: hidden;
  width: 10px;
  line-height: 0;
}

.inline {
  display: inline-block;
  width: 10px;
  height: 10px;
}

@supports (display: layout(test)) {
  .test {
    background: green;
    display: layout(test);
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
