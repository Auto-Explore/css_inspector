# css/css-masking/clip-path/animations/clip-path-animation-path.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-path.html"
}
```

## style[0]

```css

  .container {
    width: 200px;
    height: 200px;
    background-color: green;
    animation: clippath 20s steps(2, jump-end) -9.999s;
  }

  @keyframes clippath {
    0% {
      clip-path: path('M 0 200 L 0,75 A 5,5 0,0,1 150,75 L 150 200 z');
    }

    100% {
      clip-path: path('M 0 100 L 30,75 A 5,5 0,0,1 150,75 L 150 100 z');
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
