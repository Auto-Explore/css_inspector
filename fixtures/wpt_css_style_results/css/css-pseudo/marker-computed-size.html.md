# css/css-pseudo/marker-computed-size.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-computed-size.html"
}
```

## style[0]

```css

:root {
  --image: url('/images/green-100x50.png');
}
:root::after {
  /* Preload image */
  content: var(--image);
}
#target {
  font: 10px/1 Ahem;
  --content: normal;
}
#target::marker {
  content: var(--content);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
