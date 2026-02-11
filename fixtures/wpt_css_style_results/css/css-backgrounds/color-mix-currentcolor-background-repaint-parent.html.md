# css/css-backgrounds/color-mix-currentcolor-background-repaint-parent.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/color-mix-currentcolor-background-repaint-parent.html"
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
        background-color: color-mix(in hsl, transparent 0%, currentColor 100%);
        width: 100px;
        height: 100px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
