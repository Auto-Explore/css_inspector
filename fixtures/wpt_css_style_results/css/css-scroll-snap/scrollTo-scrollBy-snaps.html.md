# css/css-scroll-snap/scrollTo-scrollBy-snaps.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scrollTo-scrollBy-snaps.html"
}
```

## style[0]

```css

html {
  margin: 0px;
  overflow: scroll;
  scroll-snap-type: both mandatory;
}
div {
  position: absolute;
}
.scroller {
  overflow: scroll;
  scroll-snap-type: both mandatory;
}
#inner-scroller {
  top: 3000px;
  width: 800px;
  height: 800px;
}
.space {
  left: 0px;
  top: 0px;
  width: 4000px;
  height: 4000px;
}
.target {
  width: 600px;
  height: 600px;
  scroll-snap-align: start;
}

.left {
  left: 0px;
}
.right {
  left: 1000px;
}
.top {
  top: 0px;
}
.bottom {
  top: 1000px;
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
