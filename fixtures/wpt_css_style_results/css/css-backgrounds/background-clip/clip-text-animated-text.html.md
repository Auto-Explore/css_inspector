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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
