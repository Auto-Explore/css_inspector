# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-user-scroll.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-user-scroll.tentative.html"
}
```

## style[0]

```css

    body {
      margin: 0px;
    }

    div {
      position: absolute;
      margin: 0px;
    }

    #spacer {
      width: 200vw;
      height: 200vh;
    }

    .scroller {
      height: 400px;
      width: 400px;
      overflow: scroll;
      scroll-snap-type: both mandatory;
    }

    .snap_point {
      width: 40%;
      height: 40%;
      scroll-snap-align: start;
    }

    #snap_point_1 {
      left: 0px;
      top: 0px;
      background-color: red;
    }

    #snap_point_2 {
      top: 35%;
      left: 35%;
      background-color: orange;
    }

    #snap_point_3 {
      top: 70%;
      left: 70%;
      background-color: blue;
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
