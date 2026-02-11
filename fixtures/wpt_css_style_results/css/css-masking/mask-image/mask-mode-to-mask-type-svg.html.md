# css/css-masking/mask-image/mask-mode-to-mask-type-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-mode-to-mask-type-svg.html"
}
```

## style[0]

```css

  body {
    padding: 0;
    margin: 0;
  }

  rect {
    fill: blue;
    width: 100px;
    height: 100px;
  }

  rect.match-luminance {
    x: 10px;
    y: 10px;
    mask-mode: match-source;
    mask-image: url("#svg-luminance");
  }

  rect.match-alpha {
    x: 120px;
    y: 10px;
    mask-mode: match-source;
    mask-image: url("#svg-alpha");
  }

  rect.luminance-luminance {
    x: 10px;
    y: 120px;
    mask-mode: luminance;
    mask-image: url("#svg-luminance");
  }

  rect.luminance-alpha {
    x: 120px;
    y: 120px;
    mask-mode: luminance;
    mask-image: url("#svg-alpha");
  }

  rect.alpha-luminance {
    x: 10px;
    y: 230px;
    mask-mode: alpha;
    mask-image: url("#svg-luminance");
  }

  rect.alpha-alpha {
    x: 120px;
    y: 230px;
    mask-mode: alpha;
    mask-image: url("#svg-alpha");
  }

  #svg-luminance {
    mask-type: luminance;
  }

  #svg-alpha {
    mask-type: alpha;
  }
```

```json
{
  "errors": 27,
  "messages": [
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “x”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “y”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “x”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “y”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “x”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “y”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “x”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “y”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “x”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “y”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “x”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “y”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-type”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
