# css/CSS2/lists/counter-reset-054.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-054.xht"
}
```

## style[0]

```css

            #wrapper
            {
                counter-reset: ident 5;
            }
            #test
            {
                counter-reset: inherit;
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
