# css/css-animations/animation-fill-mode-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-fill-mode-001-manual.html"
}
```

## style[0]

```css

  div {
    animation-name: sample;
    animation-duration: 5s;
    animation-fill-mode: none;

    background-color: blue;
    height: 100px;
    width: 100px;
    position: relative;
  }
  @keyframes sample {
    from {
      background-color: yellow;
    }
    to {
      background-color: green;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
