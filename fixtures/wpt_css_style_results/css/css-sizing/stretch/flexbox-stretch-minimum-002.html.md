# css/css-sizing/stretch/flexbox-stretch-minimum-002.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/stretch/flexbox-stretch-minimum-002.html"
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
  flex-direction: column;
  width: 0;
}
#flex-item {
  min-width: stretch;
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
