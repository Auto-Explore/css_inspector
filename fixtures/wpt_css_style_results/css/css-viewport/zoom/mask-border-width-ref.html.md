# css/css-viewport/zoom/mask-border-width-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/mask-border-width-ref.html"
}
```

## style[0]

```css

    body {
        --scale: 1;
    }

    .box {
        width: calc(50px * var(--scale));
        height: calc(50px * var(--scale));
        background-color: red;
        mask-border-slice: 10;
        mask-border-source: linear-gradient(45deg, pink, blue, white, black, green);
        mask-border-width: calc(8px * var(--scale));
        margin: calc(10px * var(--scale));
    }

    .zoom {
        --scale: 2;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
