# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-programmatic-scroll.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-programmatic-scroll.tentative.html"
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
      width: 2000px;
      height: 2000px;
    }

    .scroller {
      height: 400px;
      width: 400px;
      overflow: scroll;
      scroll-snap-type: both mandatory;
    }

    .snap_point {
      width: 300px;
      height: 300px;
      scroll-snap-align: start;
    }

    #snap_point_1 {
      left: 0px;
      top: 0px;
      background-color: red;
    }

    #snap_point_2 {
      top: 300px;
      left: 300px;
      background-color: orange;
    }

    #snap_point_3 {
      left: 600px;
      top: 600px;
      background-color: blue;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
