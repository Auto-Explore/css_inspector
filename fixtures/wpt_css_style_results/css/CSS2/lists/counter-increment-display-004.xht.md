# css/CSS2/lists/counter-increment-display-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-display-004.xht"
}
```

## style[0]

```css

            body
            {
                counter-reset: hidden;
            }
            #div1
            {
                display: none;
            }
            div:before
            {
                counter-increment: hidden 1;
                content: counter(hidden);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
