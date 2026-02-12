# css/css-flexbox/percentage-heights-007.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-007.html"
}
```

## style[0]

```css

x-flexbox {
  display: flex;
  flex-direction: column;
}

x-item {
  flex: 0 0 100px;
}

x-item>div {
  width: 100px;
  height: 100%;
  background: green;
}

#reference-overlapped-red {
  position: absolute;
  background-color: red;
  width: 100px;
  height: 100px;
  z-index: -1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
