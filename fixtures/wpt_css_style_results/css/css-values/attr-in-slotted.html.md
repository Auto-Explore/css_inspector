# css/css-values/attr-in-slotted.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-in-slotted.html"
}
```

## style[0]

```css

            ::slotted(div) {
                color: red;
                color: attr(data-color type(<color>), yellow);
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
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
