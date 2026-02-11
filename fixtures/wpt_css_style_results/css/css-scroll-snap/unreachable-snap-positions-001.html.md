# css/css-scroll-snap/unreachable-snap-positions-001.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/unreachable-snap-positions-001.html"
}
```

## style[0]

```css

div {
  position: absolute;
  margin: 0px;
}
#scroller {
  height: 500px;
  width: 500px;
  overflow: hidden;
  scroll-snap-type: both mandatory;
}
#unreachable {
  width: 300px;
  height: 300px;
  top: -100px;
  left: -100px;
  background-color: blue;
  scroll-snap-align: start;
}
#reachable {
  width: 300px;
  height: 300px;
  top: 400px;
  left: 400px;
  background-color: blue;
  scroll-snap-align: start;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
