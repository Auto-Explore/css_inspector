# css/css-anchor-position/position-visibility-remove-anchors-visible.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-remove-anchors-visible.html"
}
```

## style[0]

```css

  #scroll-container {
    overflow: hidden scroll;
    width: 300px;
    height: 100px;
    font: 20px/1 Ahem;
  }

  #anchor {
    anchor-name: --a1;
    width: 100px;
    height: 100px;
    background: orange;
    font: 20px/1 Ahem;
  }

  #spacer {
    height: 100px;
  }

  #target {
    position-anchor: --a1;
    position-visibility: anchors-visible;
    position-area: bottom;
    width: 100px;
    height: 100px;
    background: green;
    position: absolute;
    inset: 0;
    font: 20px/1 Ahem;
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “overflow”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-visibility”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
