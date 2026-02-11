# css/css-layout-api/edges/scrollbar.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/edges/scrollbar.https.html"
}
```

## style[0]

```css

td { text-align: center; }

.parent {
  box-sizing: border-box;
  width: 50px;
  height: 50px;
  border: solid;
  position: relative;
  background: red;
}

@supports (display: layout(test)) {
  .parent {
    display: layout(test);
    background: initial;
  }
}

.child {
  width: 10px;
  height: 10px;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
