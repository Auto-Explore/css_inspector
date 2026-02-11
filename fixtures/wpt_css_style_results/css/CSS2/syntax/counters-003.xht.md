# css/CSS2/syntax/counters-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-003.xht"
}
```

## style[0]

```css

            body
            {
                counter-reset: chapter;
            }
            div p:before
            {
                content: counter(chapter,upper-roman);
                counter-increment: chapter;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
