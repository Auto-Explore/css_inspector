# css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in-document.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in-document.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --anchor;
    margin-left: 150vw;
    width: 100px;
    height: 100px;
    background: orange;
  }

  #spacer {
    height: 100px;
  }

  #target {
    position-anchor: --anchor;
    position-visibility: anchors-visible;
    position-area: left center;
    width: 100px;
    height: 100px;
    background: green;
    position: absolute;
    inset: 0;
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
