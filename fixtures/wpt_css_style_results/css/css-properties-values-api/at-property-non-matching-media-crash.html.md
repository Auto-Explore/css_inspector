# css/css-properties-values-api/at-property-non-matching-media-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/at-property-non-matching-media-crash.html"
}
```

## style[0]

```css

    @property --x {
      syntax: "<length>";
      inherits: false;
      initial-value: 0px;
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
