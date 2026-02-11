# css/css-masking/mask-image/mask-position-1a-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-1a-svg.html"
}
```

## style[0]

```css

  rect.frame {
    x: -0.5px;
    y: -0.5px;
    width: 101px;
    height: 101px;
    stroke: black;
    fill: none;
  }

  rect.masked {
    width: 100px;
    height: 100px;
    fill: purple;
    mask-image: url(support/50x50-opaque-blue.svg);
    mask-repeat: no-repeat;
  }

  #masked1 { mask-position: right 20% bottom 70%; }
  #masked2 { mask-position: bottom 70% right 20%; }
  #masked3 { mask-position: right 30px bottom 25px; }
  #masked4 { mask-position: bottom 25px right 30px; }
```

```json
{
  "errors": 11,
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
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
