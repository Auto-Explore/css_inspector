# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-user-root-scroll.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-user-root-scroll.tentative.html"
}
```

## style[0]

```css

    :root {
      margin: 0px;
      padding: 0px;
      scroll-snap-type: both mandatory;
    }

    div {
      position: absolute;
      margin: 0px;
    }

    #spacer {
      width: 200vw;
      height: 200vh;
    }

    .snap_point {
      width: 40vw;
      height: 40vh;
      scroll-snap-align: start;
    }

    #snap_point_1 {
      left: 0px;
      top: 0px;
      background-color: red;
    }

    #snap_point_2 {
      top: 35vh;
      left: 35vw;
      background-color: orange;
    }

    #snap_point_3 {
      left: 70vw;
      top: 70vh;
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
