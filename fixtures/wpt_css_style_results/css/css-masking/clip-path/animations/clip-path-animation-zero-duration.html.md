# css/css-masking/clip-path/animations/clip-path-animation-zero-duration.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-zero-duration.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  background-color: green;
  animation: clippath 0s;
  animation-fill-mode: both;
  clip-path: inset(25% 25%);
}
@keyframes clippath {
  0%   { clip-path: circle(50% at 50% 50%); }
  100% { clip-path: circle(20% at 20% 20%); }
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
