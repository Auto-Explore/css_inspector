# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-with-proximity-strictness.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-with-proximity-strictness.tentative.html"
}
```

## style[0]

```css

  div {
    position: absolute;
    margin: 0;
  }

  #scroller {
    height: 600px;
    width: 600px;
    overflow: hidden;
    scroll-snap-type: y proximity;
  }

  .snap {
    width: 300px;
    height: 300px;
    background-color: green;
    scroll-snap-align: start;
  }

  .area {
    width: 2000px;
    height: 2000px;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
