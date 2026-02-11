# css/css-backgrounds/background-clip/clip-text-animated-text.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-text-animated-text.html"
}
```

## style[0]

```css

  .text {
    background-color: DeepPink;
    background-clip: text;
    font-size: 50px;
    font-family: sans-serif;
    font-weight: 600;
    color: transparent;
  }

  .text p {
    animation: fade-in 0.1s both;
  }

  @keyframes fade-in {
    from { opacity: 0; }
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
      "message": "Invalid value for property “background-color”.",
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
