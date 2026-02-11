# css/CSS2/syntax/core-syntax-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/core-syntax-001.xht"
}
```

## style[0]

```css

   .test { color: green; }
   .test { test { :nested; color: yellow; background: red; }: not-nested; text-decoration: underline; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Missing property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing property name in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
