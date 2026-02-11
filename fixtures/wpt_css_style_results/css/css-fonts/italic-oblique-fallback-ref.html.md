# css/css-fonts/italic-oblique-fallback-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/italic-oblique-fallback-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: ref1;
  src: url("resources/markA.ttf");
}

@font-face {
  font-family: ref2;
  src: url("resources/markA.ttf");
  size-adjust: 50%;
}

div {
  font-size: 50px;
}

.ref1 {
  font-family: ref1, serif;
}

.ref2 {
  font-family: ref2, serif;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “size-adjust”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
