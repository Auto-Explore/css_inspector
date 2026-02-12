# css/css-highlight-api/painting/custom-highlight-painting-text-shadow-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-text-shadow-invalidation.html"
}
```

## style[0]

```css

    ::root {
      --shadow-offset-x: 15px;
      --shadow_offset-y: 20px;
    }
    ::highlight(example-highlight) {
      background-color: yellow;
      color: blue;
      text-shadow: var(--shadow-offset-x, 5px) var(--shadow-offset-y, 10px) rgba(0, 255, 0, 0.5);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
