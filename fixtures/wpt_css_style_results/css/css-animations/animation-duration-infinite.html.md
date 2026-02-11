# css/css-animations/animation-duration-infinite.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-duration-infinite.html"
}
```

## style[0]

```css


div {
  width: 100px;
  height: 100px;
  background-color: black;

  animation-name: slide;
  animation-duration: calc(infinity * 1s);
}

@keyframes slide {
  to { margin-left: 100px }
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
