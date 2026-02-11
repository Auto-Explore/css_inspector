# css/css-anchor-position/position-visibility-no-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-no-overflow.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 400px;
    height: 150px;
  }

  .anchor {
    width: 100px;
    height: 100px;
    background: orange;
    display: inline-block;
  }

  .target {
    position: absolute;
    position-visibility: no-overflow;
    position-area: block-end;
    width: 100px;
    height: 100px;
    background: red;
    top: 0;
    left: 0;
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
