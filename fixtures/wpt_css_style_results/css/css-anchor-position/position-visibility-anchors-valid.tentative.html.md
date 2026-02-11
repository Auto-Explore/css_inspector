# css/css-anchor-position/position-visibility-anchors-valid.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-valid.tentative.html"
}
```

## style[0]

```css

  .anchor {
    width: 100px;
    height: 100px;
    background: orange;
    display: inline-block;
  }

  .target {
    position: absolute;
    position-visibility: anchors-valid;
    position-area: block-end;
    width: 100px;
    height: 100px;
    background: red;
    inset: 0;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
