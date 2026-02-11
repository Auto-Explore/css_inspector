# css/css-scroll-snap/overflowing-snap-areas.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/overflowing-snap-areas.html"
}
```

## style[0]

```css

div {
  position: absolute;
}
.scroller-x {
  overflow: scroll;
  scroll-snap-type: x mandatory;
  width: 500px;
  height: 500px;
}
.scroller-y {
  overflow: scroll;
  scroll-snap-type: y mandatory;
  width: 500px;
  height: 500px;
}
.space {
  width: 4000px;
  height: 4000px;
}
.target {
  scroll-snap-align: start;
  height: 400px;
  width: 400px;
}
.large-x {
  width: 3000px;
  background-color: yellow;
}
.large-y {
  height: 2000px;
  background-color: yellow;
}
.small {
  height: 200px;
  width: 200px;
  background-color: black;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
