# css/CSS2/syntax/counters-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-008.xht"
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
                                )
                                ;
                counter-increment: chapter;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
