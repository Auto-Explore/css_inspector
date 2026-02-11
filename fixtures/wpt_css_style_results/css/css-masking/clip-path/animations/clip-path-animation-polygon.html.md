# css/css-masking/clip-path/animations/clip-path-animation-polygon.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-polygon.html"
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
    0% {
      clip-path: polygon(0% 0%, 100% 0%, 50% 100%)
    }

    100% {
      clip-path: polygon(20% 20%, 80% 20%, 50% 80%)
    }
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
