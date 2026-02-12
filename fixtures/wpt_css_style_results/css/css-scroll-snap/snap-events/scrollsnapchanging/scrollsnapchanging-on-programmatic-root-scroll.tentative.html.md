# css/css-scroll-snap/snap-events/scrollsnapchanging/scrollsnapchanging-on-programmatic-root-scroll.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchanging/scrollsnapchanging-on-programmatic-root-scroll.tentative.html"
}
```

## style[0]

```css

    :root {
      scroll-snap-type: y mandatory;
    }

    .box {
      position: absolute;
      left: 150px;
      height: 80vh;
      width: 100px;
      border: solid 1px white;
    }

    .snap {
      scroll-snap-align: start;
    }

    .blue {
      background-color: blue;
    }

    .green {
      background-color: green;
    }

    .yellow {
      background-color: yellow;
    }

    #snap_area_1 {
      top: 0px;
    }

    #snap_area_2 {
      top: calc(80vh + 2px);
    }

    #snap_area_3 {
      top: calc(160vh + 4px);
    }

    .large_space {
      height: 400vh;
      width: 400vw;
      position: absolute;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
