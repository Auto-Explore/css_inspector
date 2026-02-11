# css/css-text-decor/text-decoration-shorthands-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-shorthands-002.html"
}
```

## style[0]

```css

         /*
            This test is designed to make sure the underline is rendered
            the same, regardless of whether or not the thickness is set
            in the shorthands or the longhands form
         */
         div{
             text-decoration: green underline 100px;
         }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
