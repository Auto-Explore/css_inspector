# css/css-masking/clip-path/animations/clip-path-animation-mixed-calc.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-mixed-calc.html"
}
```

## style[0]

```css

#animated {
  width: 100px;
  height: 100px;
  font-size: 10px;
  background-color: green;
  animation: clippath 20s steps(2, jump-end) -10.05s;
}

#animated.override {
  width: 150px;
  height: 150px;
  font-size: 15px;
}

@keyframes clippath {
  0%   { clip-path: circle(10px at 50% 50%); }
  100% { clip-path: circle(calc(20% + 2em) at 50% 50%); }
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
