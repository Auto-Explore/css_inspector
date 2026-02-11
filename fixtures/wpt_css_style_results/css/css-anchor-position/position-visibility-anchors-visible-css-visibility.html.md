# css/css-anchor-position/position-visibility-anchors-visible-css-visibility.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-css-visibility.html"
}
```

## style[0]

```css

  #container {
    visibility: hidden;
  }

  #anchor {
    anchor-name: --a1;
    width: 100px;
    height: 100px;
    background: orange;
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
```

```json
{
  "errors": 5,
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
    }
  ],
  "warnings": 0
}
```
