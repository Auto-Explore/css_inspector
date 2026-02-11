# css/css-anchor-position/transform-007.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/transform-007.html"
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
      margin-left: 20px;
      transform: scale(3);
      transform-origin: top left;
      anchor-name: --a;
      background: green;
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
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
