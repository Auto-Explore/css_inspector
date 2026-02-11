# css/CSS2/lists/counter-reset-increment-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-increment-001.xht"
}
```

## style[0]

```css

            div:before
            {
                content: counter(chapter);
                counter-increment: chapter 2;
                counter-reset: chapter -10;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
