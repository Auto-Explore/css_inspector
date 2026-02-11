# css/css-scroll-snap/snap-into-covering-area.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-into-covering-area.html"
}
```

## style[0]

```css

    #scroller {
      overflow: scroll;
      height: 500px;
      width: 500px;
      background-color: blue;
      scroll-snap-type: y mandatory;
      position: absolute;
    }

    .snap_point {
      scroll-snap-align: start;
      width: 40%;
      position: relative;
      left: 30%;
    }

    .big {
      height: 1000%;
      background-color: pink;
      border: solid 1px red;
    }

    .small {
      height: 50%;
      background-color: purple;
      border: solid 1px black;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
