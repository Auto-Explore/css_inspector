# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-span.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-span.tentative.html"
}
```

## style[0]

```css

      * {
        margin: 0px;
      }
      .space {
        width: 150%;
        height: 150%;
      }
      .box {
        height: 50px;
        width: 50px;
        border: solid 1px black;
        background-color: turquoise;
      }
      .target {
        scroll-initial-target: nearest;
      }
      .scroller {
        overflow: scroll;
        height: 100px;
        width: 100px;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-initial-target”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
