# css/CSS2/cascade/at-import-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/at-import-011.xht"
}
```

## style[0]

```css

           # :unknownpseudo
           @import "support/import-red.css";
           .import { color: red; }
           p { color: green; }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
