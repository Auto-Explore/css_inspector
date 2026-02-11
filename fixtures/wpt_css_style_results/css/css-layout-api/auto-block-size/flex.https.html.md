# css/css-layout-api/auto-block-size/flex.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/auto-block-size/flex.https.html"
}
```

## style[0]

```css

.flex {
  width: 300px;
  display: flex;
  border: solid 2px;
}

.custom {
  background: red;
  box-sizing: border-box;
  border: solid 2px;
  height: 100px;
  writing-mode: vertical-rl;
}

@supports (display: layout(block-size-100)) {
  .custom-100 {
    display: layout(block-size-100);
    background: green;
  }
  .custom-50 {
    display: layout(block-size-50);
    background: green;
    flex: 1;
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
