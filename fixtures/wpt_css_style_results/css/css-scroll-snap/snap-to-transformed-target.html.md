# css/css-scroll-snap/snap-to-transformed-target.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-to-transformed-target.html"
}
```

## style[0]

```css

div {
  position: absolute;
}
#scroller {
  overflow: hidden; /* TODO: Use scrollbar-width: none */
  scroll-snap-type: x mandatory;
  width: 500px;
  height: 500px;
}
.space {
  width: 2000px;
  height: 2000px;
}
#target {
  height: 200px;
  width: 200px;
  left: 50px;
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
