# css/css-scroll-snap/snap-events/snapevents-at-document-bubble-to-window.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/snapevents-at-document-bubble-to-window.html"
}
```

## style[0]

```css

    :root {
      margin: 0;
      padding: 0;
      scroll-snap-type: y mandatory;
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
      top: 40vh;
      left: 40vw;
      background-color: orange;
    }

    #snap_point_3 {
      left: 80vw;
      top: 80vh;
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
