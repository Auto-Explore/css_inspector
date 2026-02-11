# css/css-text-decor/text-underline-offset-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-underline-offset-002.html"
}
```

## style[0]

```css

        #main{
            border-bottom: 1px solid cyan;
            display: flex;
        }
        #text, #norm{
            text-decoration-color: green;
            text-decoration-line: underline;
            text-decoration-skip-ink: none;
            font: 20px/1 Ahem;
            color: transparent;
            position: relative;
            margin-right: 10px;
        }
        #text{
            top: 10px;
            text-underline-offset: 11px;
        }
        #norm{
            top: 21px;
            text-underline-offset: 0px;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
