# css/css-scroll-snap/snap-events/scrollsnapchanging/scrollsnapchanging-after-layout-change.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchanging/scrollsnapchanging-after-layout-change.tentative.html"
}
```

## style[0]

```css

    body {
      margin: 0px;
    }
    #space {
      height: 200vh;
      width: 200vw;
    }
    .scroller {
        scroll-snap-type: x mandatory;
        overflow-x: auto;
        overflow-y: hidden;
        position: relative;
        height: 500px;
        width: 500px;
      }

      .box {
        scroll-snap-align: start;
        height: 100px;
        width: 100px;
        position: absolute;
        top: 200px;
      }

      #box1 {
        background-color: red;
      }

      #box2 {
        background-color: yellow;
        left: 200px;
      }

      #box3 {
        background-color: blue;
        left: 400px;
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
