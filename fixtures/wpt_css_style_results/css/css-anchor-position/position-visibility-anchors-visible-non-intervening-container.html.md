# css/css-anchor-position/position-visibility-anchors-visible-non-intervening-container.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-non-intervening-container.html"
}
```

## style[0]

```css

  #non-intervening-scroll-container {
    overflow: hidden;
    width: 200px;
    height: 200px;
    position: relative;
  }

  #position-container {
    position: relative;
  }

  #scroll-container {
    overflow: hidden scroll;
    width: 400px;
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
    position-area: right;
    width: 100px;
    height: 100px;
    background: green;
    position: absolute;
    top: 0;
    left: 0;
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
