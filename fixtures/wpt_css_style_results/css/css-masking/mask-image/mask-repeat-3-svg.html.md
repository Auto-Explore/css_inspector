# css/css-masking/mask-image/mask-repeat-3-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-repeat-3-svg.html"
}
```

## style[0]

```css

  rect.frame {
    x: -0.5px;
    y: -0.5px;
    width: 151px;
    height: 151px;
    stroke: black;
    fill: none;
  }

  rect.masked {
    width: 150px;
    height: 150px;
    fill: purple;
    mask-image: url(support/50x50-opaque-blue.svg);
    mask-position: left top;
  }

  #round {
    mask-repeat: round;
  }
  #round-x {
    mask-repeat: round no-repeat;
  }
  #round-y {
    mask-repeat: no-repeat round;
  }
```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Unknown property “x”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “y”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
