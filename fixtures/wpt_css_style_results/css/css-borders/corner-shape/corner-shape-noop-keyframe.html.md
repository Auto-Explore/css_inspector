# css/css-borders/corner-shape/corner-shape-noop-keyframe.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-noop-keyframe.html"
}
```

## style[0]

```css

  @keyframes noop {
    from {
      corner-shape: round;
    }
  }
  .target {
    width: 100px;
    height: 100px;
    background: green;
    border-radius: 50px;
    animation: noop 1s steps(1, start) paused;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “corner-shape”.",
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
