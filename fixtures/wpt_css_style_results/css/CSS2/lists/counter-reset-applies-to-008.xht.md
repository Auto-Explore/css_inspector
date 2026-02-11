# css/CSS2/lists/counter-reset-applies-to-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-applies-to-008.xht"
}
```

## style[0]

```css

            div
            {
                counter-reset: test 5;
                display: inline;
            }
            div:before
            {
                content: counter(test);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
