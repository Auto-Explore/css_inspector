# css/CSS2/lists/counter-reset-not-generated-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-not-generated-001.xht"
}
```

## style[0]

```css

            #div1:before
            {
                counter-reset: notgenerated 10;
                display: none;
            }
            div:before
            {
                content: counter(notgenerated);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
