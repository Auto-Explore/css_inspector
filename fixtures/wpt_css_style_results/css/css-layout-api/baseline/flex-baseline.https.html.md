# css/css-layout-api/baseline/flex-baseline.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/baseline/flex-baseline.https.html"
}
```

## style[0]

```css

.parent {
  background: red;
  display: flex;
  padding: 0 10px;
  width: 80px;
  height: 100px;
  align-items: baseline;
}

.child {
  color: red;
}

@supports (display: layout(parent)) {
  .parent {
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
