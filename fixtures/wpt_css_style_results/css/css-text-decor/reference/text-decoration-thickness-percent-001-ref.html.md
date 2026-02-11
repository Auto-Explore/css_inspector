# css/css-text-decor/reference/text-decoration-thickness-percent-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-decoration-thickness-percent-001-ref.html"
}
```

## style[0]

```css

        div {
            position: absolute;
            top: 50px;
            line-height: 100px;
        }
        u {
            display: inline-block;
            font: 20px/1 Ahem;
            width: 100px;
            text-align-last: justify;
            color: transparent;
            text-decoration: green underline;
            text-decoration-skip-ink: none;
            /* place underline exactly at the bottom of the text */
            text-underline-position: auto;
            text-underline-offset: 0px;
            text-decoration-skip-ink: none;
            text-decoration-thickness: 20px;
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
