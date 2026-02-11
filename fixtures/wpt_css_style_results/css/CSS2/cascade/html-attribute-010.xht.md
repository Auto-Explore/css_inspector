# css/CSS2/cascade/html-attribute-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/html-attribute-010.xht"
}
```

## style[0]

```css

            html, body, p, br, div
            {
                border: none;
                width: auto;
            }
            *
            {
                border: 1px solid blue;
                padding: 0;
                width: 100px;
            }
            div div
            {
                background: orange;
                height: 20px;
                width: 102px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
