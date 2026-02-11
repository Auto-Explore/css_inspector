# css/css-borders/corner-shape/corner-shape-backdrop-filter.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-backdrop-filter.html"
}
```

## style[0]

```css

    .target {
        width: 200px;
        height: 200px;
        corner-shape: squircle scoop notch bevel;
        border-radius: 40%;
    }

    .backdrop {
        background: rgba(0 128 0 / 50%);
        backdrop-filter: brightness(0);
    }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “corner-shape”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
