# css/motion/animation/offset-path-interpolation-008.html

```json
{
  "format_version": 3,
  "file": "css/motion/animation/offset-path-interpolation-008.html"
}
```

## style[0]

```css

    html {
      font-size: 16px;
    }
    .parent {
      offset-path: shape(from -5px 5px, move to 5% 1px);
    }
    .target {
      offset-path: shape(from 5px 5px, line to 10px 10%);
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
