# css/css-masking/clip-path/animations/clip-path-animation-svg-zoom.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-svg-zoom.html"
}
```

## style[0]

```css

  .clipped {
    background-color: green;
    stroke: black;
    stroke-width: 3;
    fill: red;
    animation: clippath 20s steps(2, jump-end) -9.999s;
  }

  .svg {
    width: 100px;
    height: 100px;
    zoom: 1.25;
  }

  @keyframes clippath {
    0% {
      clip-path: circle(50% at 50% 50%);
    }

    100% {
      clip-path: circle(20% at 50% 50%);
    }
  }
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
