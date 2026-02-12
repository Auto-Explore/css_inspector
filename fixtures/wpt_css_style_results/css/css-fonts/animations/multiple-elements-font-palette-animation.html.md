# css/css-fonts/animations/multiple-elements-font-palette-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/animations/multiple-elements-font-palette-animation.html"
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
  @keyframes anim {
    from {
      font-palette: normal;
    }
    to {
      font-palette: --custom;
    }
  }
  .demo {
    font-family: "COLR-test-font";
    font-size: 130px;
  }
  .anim {
    animation: anim 0.1s forwards;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
