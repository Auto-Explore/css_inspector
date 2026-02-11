# css/css-scroll-snap/snap-to-visible-areas-both.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-to-visible-areas-both.html"
}
```

## style[0]

```css

div {
  position: absolute;
  margin: 0px;
}

#scroller {
  height: 600px;
  width: 600px;
  overflow: scroll;
  scroll-snap-type: both mandatory;
}

#space {
  width: 2000px;
  height: 2000px;
}

.snap {
  width: 200px;
  height: 200px;
  background-color: blue;
  scroll-snap-align: start;
}

#left-top {
  left: 0px;
  top: 0px;
}

#left-bottom {
  left: 0px;
  top: 800px;
}

#right-top {
  left: 800px;
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
