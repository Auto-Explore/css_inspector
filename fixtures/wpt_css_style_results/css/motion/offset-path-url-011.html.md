# css/motion/offset-path-url-011.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-url-011.html"
}
```

## style[0]

```css

      #outer {
        position: relative;
        left: 50px;
        top: 50px;
        width: 300px;
        height: 100px;
      }
      #target {
        width: 50px;
        height: 50px;
        background-color: green;
        /* This behaves as 'offset-path: path("m 0 0")' */
        offset-path: url(#outer);
        offset-anchor: 50% 50%;
        offset-distance: 0%;
        border-radius: 50% 50% 0 0;
      }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-anchor”.",
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
