# css/CSS2/syntax/invalid-at-rule-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/invalid-at-rule-001.xht"
}
```

## style[0]

```css

            @badAtRule screen
            {
                div
                {
                    color: red;
                }
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
