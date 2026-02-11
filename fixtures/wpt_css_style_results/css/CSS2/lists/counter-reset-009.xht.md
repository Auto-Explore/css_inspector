# css/CSS2/lists/counter-reset-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-009.xht"
}
```

## style[0]

```css

            div
            {
                counter-reset: ident +10;
            }
            div:before
            {
                font-size: 30px;
                content: counter(ident);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
