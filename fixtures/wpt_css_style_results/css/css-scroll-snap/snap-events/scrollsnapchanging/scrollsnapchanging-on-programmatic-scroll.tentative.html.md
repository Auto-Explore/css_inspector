# css/css-scroll-snap/snap-events/scrollsnapchanging/scrollsnapchanging-on-programmatic-scroll.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchanging/scrollsnapchanging-on-programmatic-scroll.tentative.html"
}
```

## style[0]

```css

    #scroller {
      height: 400px;
      width: 400px;
      position: relative;
      overflow: scroll;
      scroll-snap-type: y mandatory;
      border: solid 1px black;
    }

    .box {
      position: absolute;
      left: 150px;
      height: 350px;
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
      top: 352px;
    }

    #snap_area_3 {
      top: 704px;
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
