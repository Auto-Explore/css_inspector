# css/css-sizing/stretch/flexbox-stretch-minimum-001.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/stretch/flexbox-stretch-minimum-001.html"
}
```

## style[0]

```css

#red {
  position: absolute;
  z-index: -1;
  width: 200px;
  height: 200px;
  background: red;
}
#flex-container {
  display: flex;
  flex-direction: row;
  height: 0;
}
#flex-item {
  min-height: stretch;
  border: 100px solid green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
