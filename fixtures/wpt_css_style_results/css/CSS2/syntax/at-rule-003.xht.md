# css/CSS2/syntax/at-rule-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/at-rule-003.xht"
}
```

## style[0]

```css

           @invalidat-block bad at rule
           this entire "at rule" should be ignored{declaration;{sub-block;}}
           div
           {
               color: green;
           }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
