# css/CSS2/lists/counter-reset-sibling-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-sibling-001.xht"
}
```

## style[0]

```css

            div
            {
                counter-increment: test 5;
            }
            #test
            {
                counter-reset: test 5;
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
