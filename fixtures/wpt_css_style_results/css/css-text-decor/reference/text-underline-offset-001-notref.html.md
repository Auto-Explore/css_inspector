# css/css-text-decor/reference/text-underline-offset-001-notref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-underline-offset-001-notref.html"
}
```

## style[0]

```css

        #main {
            margin: 2em;
            display:flex
        }
        div span {
            text-decoration: green underline;
            text-decoration-skip-ink: none;
            font: 20px/1 Ahem;
            color: transparent;
            padding-bottom: 20px;
            border: 1px dotted transparent;
            border-bottom-color: cyan;
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-*-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
