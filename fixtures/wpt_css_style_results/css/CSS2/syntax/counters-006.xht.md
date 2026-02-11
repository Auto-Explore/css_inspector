# css/CSS2/syntax/counters-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-006.xht"
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
                content:
                        counter(
                            chapter
                               )
                               ;
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
