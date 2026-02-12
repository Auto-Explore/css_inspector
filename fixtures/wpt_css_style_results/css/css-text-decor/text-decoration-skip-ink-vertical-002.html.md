# css/css-text-decor/text-decoration-skip-ink-vertical-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-skip-ink-vertical-002.html"
}
```

## style[0]

```css

           /*
            No underline should be rendered for Ahem text
            with skip-ink enabled
           */
          div{
                 font: 20px/1 Ahem;
                 color: transparent;
                 text-decoration-skip-ink: auto;
                 text-decoration: green underline;
                 text-underline-offset: -0.2em;
                 writing-mode: vertical-lr;
           }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
