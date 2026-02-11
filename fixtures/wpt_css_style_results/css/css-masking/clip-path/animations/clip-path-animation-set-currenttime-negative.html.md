# css/css-masking/clip-path/animations/clip-path-animation-set-currenttime-negative.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-set-currenttime-negative.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath 10s;
    animation-fill-mode: none;
    clip-path: inset(33% 33%);
  }

  @keyframes clippath {
    0% {
      clip-path: circle(50% at 50% 50%);
    }

    100% {
      clip-path: circle(35% at 35% 35%);
    }
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
