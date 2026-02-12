# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-after-layout-change.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-after-layout-change.tentative.html"
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
      position: absolute;
    }

    .snap_area {
      position: absolute;
      width: 100px;
      left: calc(50% - 50px);
    }

    #outer_snap_area {
      scroll-snap-align: start none;
      height: 1000px;
      background-color: blue;
    }

    #inner_snap_area {
      scroll-snap-align: start none;
      height: 100px;
      top: 100px;
      background-color: yellow;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
