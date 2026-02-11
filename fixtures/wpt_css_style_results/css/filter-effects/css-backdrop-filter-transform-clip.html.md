# css/filter-effects/css-backdrop-filter-transform-clip.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/css-backdrop-filter-transform-clip.html"
}
```

## style[0]

```css

#wrapper {
  position: absolute;
  width: 200px;
  height: 200px;
  border-radius: 50px;
  overflow: hidden;
}

#child {
  position: absolute;
  left: 50px;
  width: 300px;
  height: 300px;
  backdrop-filter: invert(100%);
}
@keyframes a {
  0% { transform: translateX(0px); }
  50% { transform: translateX(-100px); }
  100% { transform: translateX(100px); }
}
#animation {
  animation: a 2s linear paused;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
