# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-display-toggled.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-display-toggled.tentative.html"
}
```

## style[0]

```css

      #space-filler {
        width: 500px;
        height: 500px;
        border: solid 1px red;
      }
      #outer-container {
        width: 400px;
        height: 400px;
        overflow: scroll;
        border: solid 2px blue;
      }
      #inner-container {
        top: 20px;
        left: 20px;
        width: 300px;
        height: 300px;
        overflow: scroll;
        position: relative;
        border: solid 2px black;
      }
      #target {
        width: 100px;
        height: 100px;
        background-color: pink;
        scroll-initial-target: nearest;
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
