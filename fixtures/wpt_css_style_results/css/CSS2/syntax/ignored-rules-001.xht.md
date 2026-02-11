# css/CSS2/syntax/ignored-rules-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/ignored-rules-001.xht"
}
```

## style[0]

```css

            div
            {
                badrule: value;
                color: green;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “badrule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
