# css/css-anchor-position/transform-016.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/transform-016.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 200px;
    height: 200px;
    transform: translateX(0);
    background: hotpink;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: bottom right;
    width: 100%;
    height: 100%;
    background: cyan;
  }
  @keyframes anim {
    0% { transform: translateX(0) }
    50% { transform: translateX(0) }
    100% { transform: translateX(200px) }
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
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
