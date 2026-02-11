# css/css-scroll-snap/scroll-snap-stop-002-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-snap-stop-002-nested.html"
}
```

## style[0]

```css

div {
  position: absolute;
}
.scroller {
  width: 400px;
  height: 100px;
  overflow: scroll;
  scroll-snap-type: x mandatory;
}
#space {
  left: 0px;
  top: 0px;
  width: 2100px;
  height: 50px;
}
.target {
  width: 50px;
  height: 50px;
  scroll-snap-align: start;
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
