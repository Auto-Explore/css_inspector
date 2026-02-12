# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-same-targets-after-layout-changed.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-same-targets-after-layout-changed.html"
}
```

## style[0]

```css

    #scroller {
      overflow-y: scroll;
      scroll-snap-type: y mandatory;
      width: 500px;
      height: 500px;
      background-color: white;
      position: relative;
    }
    .large_space {
      position: absolute;
      height: 100vh;
      width: 100vw;
    }
    .space_filler {
      display: inline-block;
      width: 40%;
      height: 30%;
      background-color: green;
    }
    .snap_area {
      scroll-snap-align: start;
      background-color: yellow;
      position: absolute;
      width: 40%;
      height: 30%;
      top: 500px;
    }

    .left {
      left: 1px;
    }
    .right {
      left: 41%;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
