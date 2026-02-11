# css/css-anchor-position/position-visibility-anchors-visible-in-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-in-overflow.html"
}
```

## style[0]

```css

  #outer-container {
    width: 100px;
    height: 100px;
    overflow: visible;
    position: relative;
  }
  #inner-container {
    width: 100px;
    height: 100px;
    overflow: visible;
    position: relative;
  }
  #anchor {
    anchor-name: --anchor;
    position: relative;
    margin-left: 200px;
    width: 100px;
    height: 100px;
    background: orange;
  }
  #spacer {
    height: 200px;
  }
  #target-a {
    position-anchor: --anchor;
    position-visibility: anchors-visible;
    position-area: top left;
    width: 50px;
    height: 50px;
    background: green;
    position: absolute;
    inset: 0;
  }
  #target-b {
    position-anchor: --anchor;
    position-visibility: anchors-visible;
    position-area: center left;
    width: 50px;
    height: 50px;
    background: green;
    position: absolute;
    inset: 0;
  }
  #target-c {
    position-anchor: --anchor;
    position-visibility: anchors-visible;
    position-area: bottom left;
    width: 50px;
    height: 50px;
    background: green;
    position: absolute;
    inset: 0;
  }
```

```json
{
  "errors": 11,
  "messages": [
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
