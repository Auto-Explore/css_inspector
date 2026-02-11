# css/CSS2/lists/counter-increment-applies-to-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-applies-to-009.xht"
}
```

## style[0]

```css

            span
            {
                counter-increment: test 5;
                display: block;
            }
            span:before
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
