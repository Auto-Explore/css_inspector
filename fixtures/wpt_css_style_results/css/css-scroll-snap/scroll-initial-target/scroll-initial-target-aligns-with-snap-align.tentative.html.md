# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-aligns-with-snap-align.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-aligns-with-snap-align.tentative.html"
}
```

## style[0]

```css

      #space {
        width: 1000px;
        height: 1000px;
        border: solid 1px red;
      }
      #scroller {
        width: 400px;
        height: 400px;
        overflow: hidden;
        border: solid 1px blue;
        position: absolute;
      }
      #target {
        width: 100px;
        height: 100px;
        background-color: pink;
        scroll-initial-target: nearest;
        position: absolute;
        top: 400px;
        left: 400px;
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
