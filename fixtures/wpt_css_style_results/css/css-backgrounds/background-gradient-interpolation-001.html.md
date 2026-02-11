# css/css-backgrounds/background-gradient-interpolation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-gradient-interpolation-001.html"
}
```

## style[0]

```css

:root {
  --space: ;
}

div {
  height: 50px;
  width: 200px;
}

.has-gradient {
  background-image: linear-gradient(
    90deg var(--space),
    yellow 30%,
    purple 95%
  );
}

.hsl {
  --space: in hsl;
}

.oklch {
  --space: in oklch;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Missing value for property “--space”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
