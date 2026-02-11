# css/css-masking/clip-path/animations/clip-path-animation-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-overflow.html"
}
```

## style[0]

```css

.container {
  width: 200px;
  height: 200px;
  background-color: green;
  border: 20px solid black;
  animation: clippath 20s steps(2, jump-end) -9.999s;
}
@keyframes clippath {
  0%   { clip-path: circle(130px at 130px 130px); }
  100% { clip-path: circle(110px at 110px 110px); }
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
