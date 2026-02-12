# css/css-scroll-snap/scroll-on-large-element-not-covering-snapport.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-on-large-element-not-covering-snapport.html"
}
```

## style[0]

```css

div {
  position: absolute;
  width: 100%;
}
#scroller {
  left: 10px;
  height: 200px;
  width: 100px;
  overflow-y: scroll;
  overflow-x: hidden;
  scroll-snap-type: y mandatory;
}
.snap {
  scroll-snap-align: start;
  background-color: green;
}
.target {
  background-color: red;
  border-radius: 100%;
  height: 88px;
  top: 536px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
