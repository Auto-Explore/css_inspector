# css/motion/offset-path-url-008.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-url-008.html"
}
```

## style[0]

```css

      #outer {
        width: 300px;
        height: 100px;
        padding: 50px;
        border: 50px solid black;
      }
      #target {
        position: relative;
        /**
         * offset-path:url() references to the svg element and treats coord-box
         * as its user coordinate system. And the svg element always gives it
         * the offset starting position (based on the svg attributes). So the
         * transformed box should ignore its current position from CSS layout.
         * Intentionally use left property to make sure we ignore it.
         */
        left: 100px;
        width: 50px;
        height: 50px;
        background-color: green;
        offset-path: url(#svgRect) border-box;
        offset-distance: 0%;
        border-radius: 50% 50% 0 0;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-distance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
