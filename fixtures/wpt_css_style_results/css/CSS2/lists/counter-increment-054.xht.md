# css/CSS2/lists/counter-increment-054.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-054.xht"
}
```

## style[0]

```css

            #wrapper
            {
                counter-increment: ident 5;
            }
            #test
            {
                counter-increment: inherit;
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
