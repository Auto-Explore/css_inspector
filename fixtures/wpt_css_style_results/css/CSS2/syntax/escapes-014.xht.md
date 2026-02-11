# css/CSS2/syntax/escapes-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escapes-014.xht"
}
```

## style[0]

```css

   .test { color: white; background: green; }
   .test { color:\0020yellow; background:\0020red; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
