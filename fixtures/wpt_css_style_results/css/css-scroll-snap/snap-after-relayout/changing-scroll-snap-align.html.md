# css/css-scroll-snap/snap-after-relayout/changing-scroll-snap-align.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/changing-scroll-snap-align.html"
}
```

## style[0]

```css

div {
  position: absolute;
  margin: 0;
}

#scroller {
  height: 200px;
  width: 200px;
  overflow: hidden;
  scroll-snap-type: both mandatory;
}

#initial-target {
  width: 300px;
  height: 300px;
  top: 100px;
  left: 100px;
  background-color: green;
  scroll-snap-align: start;
}

#other-target {
  width: 300px;
  height: 300px;
  top: 300px;
  left: 300px;
  background-color: red;
  scroll-snap-align: start;
}

.area {
  width: 2000px;
  height: 2000px;
}

.snap-area {
  scroll-snap-align: start !important;
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
