# css/css-scroll-snap/snap-to-visible-areas-margin-x-axis.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-to-visible-areas-margin-x-axis.html"
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
  scroll-snap-type: x mandatory;
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

#left-visible {
  left: 0px;
  top: 0px;
}

#right-visible {
  left: 800px;
  top: 0px;
}

#middle-margin-visible {
  left: 700px;
  top: 800px;
  /* 300px makes snap area visible with margin but non-visible without it */
  scroll-margin-top: 300px;
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
