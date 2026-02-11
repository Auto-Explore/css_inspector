# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-interrupted-scroll.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-on-interrupted-scroll.tentative.html"
}
```

## style[0]

```css

      #container {
        overflow-y: scroll;
        height: 500px;
        width: 300px;
        border: solid 1px black;
        position: absolute;
        scroll-snap-type: y mandatory;
      }
      .snap_area {
        scroll-snap-align: start;
        height: 400px;
        width: 200px;
        left: 50px;
        position: absolute;
      }
      #area1 {
        background-color: blue;
      }
      #area2 {
        top: 400px;
        background-color: yellow;
      }
      #area3 {
        top: 800px;
        background-color: green;
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
