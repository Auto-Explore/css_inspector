# css/css-flexbox/percentage-padding-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-padding-001.html"
}
```

## style[0]

```css

x-flexbox {
  display: flex;
  height: 50px;
  width: 500px;
}

x-item {
  padding-left: 10%;
  background: blue;
  position: relative; /* Just so offsetLeft returns distance between x-item and div */
}

div {
  height: 50px;
  width: 50px;
  background: orange;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
