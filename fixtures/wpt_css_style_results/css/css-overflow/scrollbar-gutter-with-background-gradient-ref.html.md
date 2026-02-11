# css/css-overflow/scrollbar-gutter-with-background-gradient-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-with-background-gradient-ref.html"
}
```

## style[0]

```css

  html {
    scrollbar-gutter: stable both-edges;
    background-image: linear-gradient(to right, green, blue);
    position: fixed;
    inset: 0;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “scrollbar-gutter”.",
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
