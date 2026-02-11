# css/css-masking/clip-path/animations/clip-path-animation-filter.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-filter.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  background-color: green;
  filter: blur(5px);
  animation: clippath 20s steps(2, jump-end) -9.999s;
}
@keyframes clippath {
  0%   { clip-path: circle(50% at 50% 50%); }
  100% { clip-path: circle(100% at 100% 100%); }
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
      "message": "Invalid value for property “filter”.",
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
