# css/css-view-transitions/nested/compute-explicit-name-nested-vt-names.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/compute-explicit-name-nested-vt-names.tentative.html"
}
```

## style[0]

```css

    ::view-transition-group(yellow) {
        opacity: 0;
        background: yellow;
        animation: none;
    }

    .yellow {
        view-transition-name: yellow;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
