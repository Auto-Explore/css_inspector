# css/css-conditional/at-supports-043.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-043.html"
}
```

## style[0]

```css

            div {
                background-color: green;
                height: 100px;
                width: 100px;
            }

            @supports ( ( background-color: red ) or( background-color: green ) ) {
                div { background-color: red; }
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
