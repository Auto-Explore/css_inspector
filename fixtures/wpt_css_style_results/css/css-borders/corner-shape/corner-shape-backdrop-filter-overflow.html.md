# css/css-borders/corner-shape/corner-shape-backdrop-filter-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-backdrop-filter-overflow.html"
}
```

## style[0]

```css

    .target {
        width: 200px;
        height: 200px;
        corner-shape: superellipse(1.8) scoop superellipse(-5) bevel;
        border-radius: 25%;
        position: relative;
    }

    .backdrop {
        background: rgba(0 128 0 / 50%);
        backdrop-filter: brightness(0);
    }

    .overflow {
        position: absolute;
        left: 100px;
        top: 100px;
        width: 200px;
        height: 200px;
        background: rebeccapurple;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “corner-shape”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
