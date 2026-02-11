# css/css-borders/tentative/border-shape/border-shape-stroke-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-stroke-invalidation.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #changer {
    width: 200px;
    height: 200px;
    border-shape: circle(50% at 50% 50%);
    stroke: blue;
    stroke-width: 50px;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “border-shape”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
