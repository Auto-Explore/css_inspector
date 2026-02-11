# css/CSS2/cascade/at-import-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/at-import-010.xht"
}
```

## style[0]

```css

           # { background: red; }
           :unknownpseudo { background: red; }
           @import "support/import-green.css";
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
