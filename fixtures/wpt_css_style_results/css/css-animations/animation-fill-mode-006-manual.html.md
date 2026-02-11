# css/css-animations/animation-fill-mode-006-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-fill-mode-006-manual.html"
}
```

## style[0]

```css

  div::before {
    animation-name: sample;
    animation-duration: 5s;
    animation-fill-mode: forwards;

    background-color: blue;
    content: "Filler Text";
    display: block;
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
