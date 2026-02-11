# css/CSS2/syntax/malformed-decl-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/malformed-decl-003.xht"
}
```

## style[0]

```css

            div
            {
                color: green;
                color:
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
