# css/css-scroll-snap/snap-after-initial-layout/writing-mode-vertical-lr.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-initial-layout/writing-mode-vertical-lr.html"
}
```

## style[0]

```css

div {
  position: absolute;
  margin: 0;
}

#scroller {
  height: 500px;
  width: 500px;
  overflow: hidden;
  scroll-snap-type: both mandatory;
  writing-mode: vertical-lr;
}

#close-target {
  width: 200px;
  height: 200px;
  border: solid green 50px;
  top: 50px;
  left: 150px;
  margin: 50px;
  background-color: green;
  scroll-snap-align: end start;
}

#far-target {
  width: 300px;
  height: 300px;
  top: 100px;
  left: 500px;
  background-color: red;
  scroll-snap-align: end start;
}

.area {
  width: 2000px;
  height: 2000px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scroll-snap-align”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scroll-snap-align”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
