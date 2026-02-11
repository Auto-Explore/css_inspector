# css/CSS2/lists/counter-increment-auto-reset-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-auto-reset-001.xht"
}
```

## style[0]

```css

            div1:before
            {
                counter-increment: outofscope 10;
            }
            #div2:before
            {
                content: counter(outofscope);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
