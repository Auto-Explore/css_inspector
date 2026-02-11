# css/css-shapes/shape-outside-invalid-circle-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside-invalid-circle-001.html"
}
```

## style[0]

```css

    .circle {
        float: left;
        width: 100px;
        height: 100px;
    }
    #circle1 {
        shape-outside: circle(auto);
    }
    #circle2 {
        shape-outside: circle(inherit);
    }
    #circle3 {
        shape-outside: circle(#FFFFFF);
    }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
