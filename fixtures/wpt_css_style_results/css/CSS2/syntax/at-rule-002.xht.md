# css/CSS2/syntax/at-rule-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/at-rule-002.xht"
}
```

## style[0]

```css

           @invalidimport bad at rule
           this entire "at rule" should be ignored;
           div
           {
               color: green;
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
