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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
