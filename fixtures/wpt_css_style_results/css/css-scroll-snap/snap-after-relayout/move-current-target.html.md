# css/css-scroll-snap/snap-after-relayout/move-current-target.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/move-current-target.html"
}
```

## style[0]

```css

div {
  position: relative;
  margin: 0;
}

#block {
  height: 100px;
  width: 100px;
}

#scroller {
  height: 500px;
  width: 500px;
  overflow: hidden;
  scroll-snap-type: both mandatory;
}

#initial-target {
  width: 300px;
  height: 300px;
  left: 100px;
  top: 0;
  transform: none;
  background-color: green;
  scroll-snap-align: start;
}

#other-target {
  width: 300px;
  height: 300px;
  left: 300px;
  background-color: red;
  scroll-snap-align: start;
}

.area {
  width: 2000px;
  height: 2000px;
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
