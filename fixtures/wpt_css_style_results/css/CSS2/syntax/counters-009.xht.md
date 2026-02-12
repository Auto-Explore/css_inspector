# css/CSS2/syntax/counters-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-009.xht"
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
                        counters(
                            chapter
                                ,
                                    "."
                                        ,
                                            upper-roman
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
