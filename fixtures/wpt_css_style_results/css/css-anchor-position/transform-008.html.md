# css/css-anchor-position/transform-008.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/transform-008.html"
}
```

## style[0]

```css

    .anchored {
      position: absolute;
      position-anchor: --a;
      background: green;
    }
    #anchor {
      width: 20px;
      height: 20px;
      margin-left: 10px;
      anchor-name: --a;
      background: green;
    }
    #transformed {
      transform: scale(3);
      transform-origin: top left;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
