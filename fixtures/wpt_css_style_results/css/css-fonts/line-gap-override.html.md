# css/css-fonts/line-gap-override.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/line-gap-override.html"
}
```

## style[0]

```css

@font-face {
  font-family: Ahem;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  line-gap-override: 100%;
}

.target {
  font: 20px Ahem;
  color: green;
  position: absolute;
  top: 10px;
  left: 10px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “line-gap-override”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
