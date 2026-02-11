# css/css-borders/corner-shape/corner-shape-bevel-overflow-composite-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-bevel-overflow-composite-ref.html"
}
```

## style[0]

```css

    .bevel {
        width: 100px;
        height: 100px;
        border-radius: 50px;
        border: 0px solid black;
        transform: translate3d(10px, 10px, 10px);
        clip-path: polygon(0px 50px, 50px 100px, 100px 50px, 50px 0px);
        transform-style: preserve-3d;
    }

    .green {
        will-change: transform;
        width: 200px;
        height: 200px;
        background: green;
        transform: translate3d(10px, 10px, 10px);
        transform-style: preserve-3d;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
