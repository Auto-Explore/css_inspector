# css/CSS2/syntax/malformed-decl-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/malformed-decl-008.xht"
}
```

## style[0]

```css

            div
            {
                :red;
                color:green;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing property name in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
