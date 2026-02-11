# css/css-pseudo/marker-variable-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-variable-ref.html"
}
```

## style[0]

```css

      .firstTest::marker {
        color: rgb(255 119 0 / 0.75);
      }

      .secondTest::marker {
        color: rgb(255 119 0);
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
