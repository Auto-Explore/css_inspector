# css/CSS2/syntax/ignored-rules-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/ignored-rules-006.xht"
}
```

## style[0]

```css

            div
            {
                color: green;
                color: /red;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
