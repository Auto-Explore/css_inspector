# css/css-scroll-snap/scroll-snap-nested-snap-area-layout-changed.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-snap-nested-snap-area-layout-changed.html"
}
```

## style[0]

```css

    #scroller {
      overflow: scroll;
      scroll-snap-type: y mandatory;
      height: 200px;
      width: 200px;
      border: solid 1px;
      position:absolute;
    }
    .snap_area {
      position: absolute;
      width: 100px;
      left: calc(50% - 50px);
    }
    #outer_snap_area {
      scroll-snap-align: start;
      height: 1000px;
      background-color: blue;
    }
    #inner_snap_area {
      scroll-snap-align: start;
      height: 100px;
      top: 100px;
      background-color: yellow;
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
