# css/css-ui/outline-negative-offset-composited-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-negative-offset-composited-scroll.html"
}
```

## style[0]

```css

#container {
  width: 100px;
  height: 100px;
  background: red;
  will-change: transform;
  overflow: scroll;
  outline: solid 50px green;
  outline-offset: -50px;
}
#child {
  position: relative;
  will-change: transform;
  height: 1000px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
