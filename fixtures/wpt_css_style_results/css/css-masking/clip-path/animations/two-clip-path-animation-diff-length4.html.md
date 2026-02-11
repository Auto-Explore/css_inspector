# css/css-masking/clip-path/animations/two-clip-path-animation-diff-length4.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/two-clip-path-animation-diff-length4.html"
}
```

## style[0]

```css

  #container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath2 0.5s;
  }

  @keyframes clippath2 {
    0% {
      clip-path: inset(10px 10px);
    }

    1% {
      clip-path: inset(10px 10px);
    }

    100% {
      clip-path: inset(40px 40px);
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
