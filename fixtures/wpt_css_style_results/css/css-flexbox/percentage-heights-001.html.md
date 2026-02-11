# css/css-flexbox/percentage-heights-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-001.html"
}
```

## style[0]

```css

.rect {
  width: 50px;
  height: 50px;
  background-color: blue;
}

.flexbox {
  width: 50px;
  outline: 3px solid black;
}


.flexbox > * {
  min-height: 0;
  min-width: 0;
}

.flexbox > div > div {
  overflow: hidden;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
