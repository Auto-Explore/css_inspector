# css/css-anchor-position/position-visibility-anchors-visible-change-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-change-anchor.html"
}
```

## style[0]

```css

  #scroll-container {
    overflow: hidden scroll;
    width: 300px;
    height: 100px;
  }

  .anchor {
    width: 100px;
    height: 100px;
    background: orange;
    display: inline-block;
  }

  #anchor1 {
    height: 200px;
    anchor-name: --a1;
  }

  #anchor2 {
    anchor-name: --a2;
  }

  #target {
    position-anchor: --a2;
    position-visibility: anchors-visible;
    position-area: bottom center;
    width: 100px;
    height: 100px;
    background: green;
    position: absolute;
    inset: 0;
  }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “overflow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
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
