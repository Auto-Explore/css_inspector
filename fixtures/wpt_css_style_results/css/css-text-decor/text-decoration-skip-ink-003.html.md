# css/css-text-decor/text-decoration-skip-ink-003.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-skip-ink-003.html"
}
```

## style[0]

```css

         /*
            This test ensures that the underline offset is taken into account
            when calculating the skip-ink. No skip-ink should be applied
            because the underline is being moved below the text.
          */
         div{
             font: 20px/1 Ahem;
             color: rgba(255,255,0,0.25);
             text-decoration: green underline;
             text-decoration-skip-ink: auto;
             text-underline-offset: .5em;
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
