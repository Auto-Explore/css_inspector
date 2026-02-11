# css/CSS2/syntax/counters-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-001.xht"
}
```

## style[0]

```css

            body
            {
                counter-reset: chapter;
            }
            div:before
            {
                content: counter(chapter);
                color: green;
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
