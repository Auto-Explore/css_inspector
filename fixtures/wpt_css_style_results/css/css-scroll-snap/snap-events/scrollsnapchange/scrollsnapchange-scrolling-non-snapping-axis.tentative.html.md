# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-scrolling-non-snapping-axis.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-scrolling-non-snapping-axis.tentative.html"
}
```

## style[0]

```css

      .scroller {
        overflow: scroll;
        width: 200px;
        height: 200px;
        border: solid 1px black;
        scroll-snap-type: y mandatory;
        position: absolute;
        resize: both;
      }
      .snaparea {
        scroll-snap-align: start;
        height: 50px;
        width: 50px;
        color: white;
        margin-top: 100px;
        background-color: purple;
      }
      .space {
        height: 300vh;
        width: 300vw;
        position: absolute;
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
