# css/css-anchor-position/position-visibility-anchors-visible-stacked-child.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-stacked-child.tentative.html"
}
```

## style[0]

```css

  #scroll-container {
    overflow: hidden scroll;
    width: 300px;
    height: 100px;
  }

  #anchor {
    anchor-name: --a1;
    width: 100px;
    height: 100px;
    background: orange;
  }

  #spacer {
    height: 100px;
  }

  #target {
    position-anchor: --a1;
    position-visibility: anchors-visible;
    position-area: bottom right;
    width: 100px;
    height: 100px;
    background: red;
    position: absolute;
    top: 0;
    left: 0;
  }
  #stacking-child {
    /* stacking context */
    z-index: 1;
    width: 100px;
    height: 100px;
    background: maroon;
    position: absolute;
    top: 25px;
    left: 25px;
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
