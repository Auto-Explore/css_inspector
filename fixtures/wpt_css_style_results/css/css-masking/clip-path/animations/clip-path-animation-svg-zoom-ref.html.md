# css/css-masking/clip-path/animations/clip-path-animation-svg-zoom-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-svg-zoom-ref.html"
}
```

## style[0]

```css

  .clipped {
    background-color: green;
    stroke: black;
    stroke-width: 3;
    fill: red;
    clip-path: circle(35% at 50% 50%);
  }

  .svg {
    width: 100px;
    height: 100px;
    zoom: 1.25;
  }

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
