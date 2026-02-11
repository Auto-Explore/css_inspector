# css/CSS2/lists/counter-increment-display-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-display-002.xht"
}
```

## style[0]

```css

            div
            {
                counter-increment: hidden 1;
            }
            #div1
            {
                display: none;
            }
            div:before
            {
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
