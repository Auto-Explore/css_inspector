# css/CSS2/syntax/counters-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-004.xht"
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
                content: counters(chapter,".");
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
