# css/css-scroll-snap/scroll-padding.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-padding.html"
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
#target {
  width: 300px;
  height: 300px;
  background-color: blue;
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
