# css/css-borders/corner-shape/corner-shape-backdrop-filter-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-backdrop-filter-ref.html"
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

    .ref {
        background: rgb(0 64 0);
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “corner-shape”.",
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
