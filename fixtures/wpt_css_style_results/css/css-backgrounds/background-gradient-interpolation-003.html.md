# css/css-backgrounds/background-gradient-interpolation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-gradient-interpolation-003.html"
}
```

## style[0]

```css

:root {
  --space: ;
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

.text {
  font: 50px/1 Ahem;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  text-fill-color: transparent;
  width: fit-content;
  margin: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
