# css/css-scroll-snap/snap-intended-direction.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-intended-direction.html"
}
```

## style[0]

```css

div {
  position: absolute;
  margin: 0px;
}
#scroller {
  width: 200px;
  height: 100px;
  overflow: scroll;
  scroll-snap-type: x mandatory;
}
.snap {
  scroll-snap-align: start;
  background: green;
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
