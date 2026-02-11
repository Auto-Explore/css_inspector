# css/css-transforms/transform-scale-hittest.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-scale-hittest.html"
}
```

## style[0]

```css

#normal {
  width: 100px;
  height: 10px;
  position: absolute;
  top: 0px;
}

#scaled {
  width: 1px;
  height: 1px;
  transform: scaleX(100) scaleY(100);
  transform-origin: 0px 0px;
  position: absolute;
  top: 10px;
  z-index: 1; /* Hit test #scaled before #normal */
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
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
