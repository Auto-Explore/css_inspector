# css/css-text-decor/text-decoration-skip-ink-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-skip-ink-002.html"
}
```

## style[0]

```css

         /*
            No underline should be rendered for Ahem text when
            text-decoration-skip-ink is applied.
          */
         div{
             font: 20px/1 Ahem;
             text-decoration: green underline;
             text-decoration-skip-ink: auto;
             color: transparent;
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
