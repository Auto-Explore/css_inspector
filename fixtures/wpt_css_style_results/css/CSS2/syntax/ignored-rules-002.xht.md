# css/CSS2/syntax/ignored-rules-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/ignored-rules-002.xht"
}
```

## style[0]

```css

            div
            {
                color: green;
                badrule: value;
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
