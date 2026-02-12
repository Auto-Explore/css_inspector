# css/CSS2/syntax/core-syntax-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/core-syntax-006.xht"
}
```

## style[0]

```css

   body { color: green; }
   p { background: red ! fail; color: yellow ! fail }
   div { background: red ! important fail; color: yellow ! important fail }
  
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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
