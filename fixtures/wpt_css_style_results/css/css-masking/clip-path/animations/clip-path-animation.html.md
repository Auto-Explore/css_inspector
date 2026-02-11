# css/css-masking/clip-path/animations/clip-path-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  background-color: green;
  animation: clippath 20s steps(2, jump-end) -9.999s;
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
