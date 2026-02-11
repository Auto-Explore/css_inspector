# css/css-scroll-snap/scroll-snap-stop-001.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-snap-stop-001.html"
}
```

## style[0]

```css

div {
  position: absolute;
}
#scroller {
  width: 400px;
  height: 400px;
  overflow: scroll;
  scroll-snap-type: both mandatory;
}
#space {
  left: 0px;
  top: 0px;
  width: 2100px;
  height: 2100px;
}
.target {
  width: 50px;
  height: 50px;
  scroll-snap-align: start;
}
.origin {
  left: 0px;
  top: 0px;
}
.always-stop {
  left: 100px;
  top: 0px;
  scroll-snap-stop: always;
}
.closer {
  left: 200px;
  top: 0px;
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
