# css/css-scroll-snap/input/snap-area-overflow-boundary-viewport-covering.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/input/snap-area-overflow-boundary-viewport-covering.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  #scroller {
    scroll-snap-type: block mandatory;
    overflow-y: scroll;
    height:  400px;
    width: 400px
  }
  #space {
    width: 200px;
    height: 4000px;
  }
  .box {
    scroll-snap-align: start;
    background: #ccccff;
    margin-bottom:  10px;
    width: 300px;
    height: 500px;
    position:  relative;
  }
  .header {
    top: 0;
    position:  absolute;
  }
  .footer {
    bottom:  0;
    position:  absolute;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
