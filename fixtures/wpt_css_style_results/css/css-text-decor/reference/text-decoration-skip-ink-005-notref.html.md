# css/css-text-decor/reference/text-decoration-skip-ink-005-notref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-decoration-skip-ink-005-notref.html"
}
```

## style[0]

```css

         @font-face {
             font-family: mplus;
             src: url(/fonts/mplus-1p-regular.woff) format("woff");
         }
         div{
             font: 50px mplus, sans-serif;
             color: #00008080;
             letter-spacing: 10px;
             text-decoration: green underline;
             text-underline-offset: -3px;
             text-decoration-thickness: 3px;
             text-decoration-skip-ink: none;
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
