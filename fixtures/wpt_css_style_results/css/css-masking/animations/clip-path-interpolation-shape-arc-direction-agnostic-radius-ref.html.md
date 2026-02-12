# css/css-masking/animations/clip-path-interpolation-shape-arc-direction-agnostic-radius-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/clip-path-interpolation-shape-arc-direction-agnostic-radius-ref.html"
}
```

## style[0]

```css

@keyframes animate-shape {
  from {
    clip-path: shape(from 40px 100px, arc to 200px 100px of 50% 50% small cw, arc to 0 100px of 30% 30% small cw);
  }
  to {
    clip-path: shape(from 40px 100px, arc to 200px 100px of 30% 30% small cw, arc to 0 100px of calc(10px + 45%) calc(10px + 45%) small cw);
  }
}
#shape {
  width: calc(500px / sqrt(2));
  height: calc(500px / sqrt(2));
  background: green;
  animation: animate-shape 100s;
  animation-play-state: paused;
  animation-timing-function: steps(2, start);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
