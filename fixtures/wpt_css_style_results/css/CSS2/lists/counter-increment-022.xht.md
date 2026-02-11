# css/CSS2/lists/counter-increment-022.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-022.xht"
}
```

## style[0]

```css

            div
            {
                counter-increment: ident +10 ident +10;
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
