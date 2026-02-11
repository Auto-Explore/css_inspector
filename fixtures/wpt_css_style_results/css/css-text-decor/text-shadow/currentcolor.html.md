# css/css-text-decor/text-shadow/currentcolor.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-shadow/currentcolor.html"
}
```

## style[0]

```css

     body {
         color: red;
         text-shadow: 0 0 10px currentcolor;
     }

     p {
         text-shadow: inherit;
         color: green;
         font: 1.5em Georgia, serif;
     }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
