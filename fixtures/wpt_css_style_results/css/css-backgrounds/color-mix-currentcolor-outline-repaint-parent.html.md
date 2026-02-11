# css/css-backgrounds/color-mix-currentcolor-outline-repaint-parent.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/color-mix-currentcolor-outline-repaint-parent.html"
}
```

## style[0]

```css

    #container {
        color: red;
    }

    #container.green {
        color: green;
    }

    #target {
        outline: 50px solid color-mix(in hsl, transparent 0%, currentColor 100%);
        outline-offset: -50px;
        width: 100px;
        height: 100px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
