# css/css-backgrounds/color-mix-currentcolor-border-repaint.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/color-mix-currentcolor-border-repaint.html"
}
```

## style[0]

```css

    #target {
        color: red;
        border: 50px solid color-mix(in hsl, transparent 0%, currentColor 100%);
        width: 0;
    }

    #target.green {
        color: green;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
