# css/CSS2/cascade-import/cascade-import-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade-import/cascade-import-001.xht"
}
```

## style[0]

```css

   @import url(data:text/css,@import%20url\(data:text/css,.test%2520%257B%2520background:%2520maroon;%2520color:%2520white;%2520%257D\);%0D%0A.test.test%20%7B%20background:%20green;%20color:%20white;%20%7D);
   p { color: yellow; background: red; }
  
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
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
