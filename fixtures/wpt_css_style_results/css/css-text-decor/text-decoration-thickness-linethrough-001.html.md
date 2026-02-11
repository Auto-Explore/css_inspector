# css/css-text-decor/text-decoration-thickness-linethrough-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-thickness-linethrough-001.html"
}
```

## style[0]

```css

        div{
            overflow: hidden;
            height: 1em;
            width: 4em;
            background-color: red;
            font: 20px/1 Ahem;
            color: transparent;
            text-decoration: green line-through;
            text-decoration-skip-ink: none;
            /* We make the text decoration just a bit thicker than the div's height, so that
             * it will entirely cover the div's content-box (making it fully green) as long
             * as the line-through is approximately centered, vertically.
             */
            text-decoration-thickness: 1.1em;
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
