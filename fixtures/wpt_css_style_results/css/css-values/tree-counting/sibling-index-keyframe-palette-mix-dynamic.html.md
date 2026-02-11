# css/css-values/tree-counting/sibling-index-keyframe-palette-mix-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-palette-mix-dynamic.html"
}
```

## style[0]

```css

  @font-face {
    font-family: "COLR-test-font";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
  }

  @font-palette-values --MyFirstPalette {
    font-family: "COLR-test-font";
    base-palette: 1;
  }

  @font-palette-values --MySecondPalette {
    font-family: "COLR-test-font";
    base-palette: 2;
  }

  @keyframes --anim {
    from {
      font-palette: palette-mix(in lch, --MyFirstPalette calc(5% * sibling-index()), --MySecondPalette 80%);
    }
    to {
      font-palette: palette-mix(in lch, --MyFirstPalette 90%, --MySecondPalette 10%);
    }
  }
  #target {
    animation: --anim 1000s step-end;
  }
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
