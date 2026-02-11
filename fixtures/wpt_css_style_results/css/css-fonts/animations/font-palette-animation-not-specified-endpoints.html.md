# css/css-fonts/animations/font-palette-animation-not-specified-endpoints.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/animations/font-palette-animation-not-specified-endpoints.html"
}
```

## style[0]

```css

  @font-face {
    font-family: "COLR-test-font";
    src: url("../resources/COLR-palettes-test-font.ttf") format("truetype");
  }
  @font-palette-values --custom {
    font-family: "COLR-test-font";
    base-palette: 3;
  }
  @keyframes animFrom {
    from {
      font-palette: --custom;
    }
  }
  @keyframes animTo {
    to {
      font-palette: --custom;
    }
  }
  .demo {
    font-family: "COLR-test-font";
    font-size: 130px;
  }
  .animFrom {
    animation: animFrom 0.1s forwards;
  }
  .animTo {
    animation: animTo 0.1s forwards;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “font-palette”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “font-palette”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
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
