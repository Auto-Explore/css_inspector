# css/CSS2/syntax/at-rule-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/at-rule-009.xht"
}
```

## style[0]

```css

           1badselector
           {
               someprop:someval;
           }
           @import "support/at-rule-green.css";
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “someprop”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
