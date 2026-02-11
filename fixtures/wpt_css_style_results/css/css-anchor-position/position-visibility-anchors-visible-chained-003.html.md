# css/css-anchor-position/position-visibility-anchors-visible-chained-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-chained-003.html"
}
```

## style[0]

```css

  #scroll-container {
    overflow: hidden scroll;
    width: 300px;
    height: 100px;
  }

  #anchor1 {
    anchor-name: --a1;
    width: 100px;
    height: 100px;
    background: orange;
  }

  .spacer {
    height: 100px;
  }

  #anchor2 {
    anchor-name: --a2;
    position-anchor: --a1;
  }

  #anchor3 {
    anchor-name: --a3;
    position-anchor: --a2;
  }

  #anchor4 {
    anchor-name: --a4;
    position-anchor: --a3;
  }

  .anchored {
    position-visibility: anchors-visible;
    position-area: bottom;
    width: 100px;
    height: 50px;
    background: red;
    position: absolute;
    top: 0;
    left: 0;
  }

  #target {
    position-anchor: --a4;
  }
```

```json
{
  "errors": 12,
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
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
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
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
