# css/css-masking/animations/clip-path-interpolation-shape-arc-direction-agnostic-radius.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/clip-path-interpolation-shape-arc-direction-agnostic-radius.html"
}
```

## style[0]

```css

@keyframes animate-shape {
  from {
    clip-path: shape(from 40px 100px, arc to 200px 100px of 50% small cw, arc to 0 100px of 30% small cw);
  }
  to {
    clip-path: shape(from 40px 100px, arc to 200px 100px of 30% small cw, arc to 0 100px of calc(10px + 45%) small cw);
  }
}
#shape {
  width: 400px;
  height: 300px;
  background: green;
  animation: animate-shape 100s;
  animation-play-state: paused;
  animation-timing-function: steps(2, start);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
