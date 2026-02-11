# css/CSS2/syntax/malformed-decl-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/malformed-decl-004.xht"
}
```

## style[0]

```css

            div
            {
                color: red;
                color: ;
                color: green;
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
