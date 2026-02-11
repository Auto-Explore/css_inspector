# css/css-properties-values-api/resolved-color-value.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/resolved-color-value.html"
}
```

## style[0]

```css

  @property --color {
    inherits: false;
    initial-value: black;
    syntax: "<color>";
  }
  #target {
    --color: color(srgb 0 sibling-index() 0);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
