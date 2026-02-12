# css/css-animations/animation-base-response-002.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-base-response-002.html"
}
```

## style[0]

```css

  @keyframes font_size_animation {
    from { font-size: 10px; }
    to { font-size: 20px; }
  }
  :root {
    font-size: 1px;
    animation: font_size_animation steps(2, end) 1000s -500s;
  }

  #target1 {
    width: 1rem;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
