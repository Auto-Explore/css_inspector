# css/CSS2/syntax/counters-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-010.xht"
}
```

## style[0]

```css

            body
            {
                counter-reset: chapter;
            }
            #p1
            {
                counter-reset: chapter;
            }
            div p:before
            {
                content: counter(chapter);
                counter-increment: chapter;
            }
            #p1:before
            {
                content: counters(chapter, ".");
                counter-increment: chapter;
            }
            #p2:before
            {
                content: counters(chapter, ".");
                counter-increment: chapter;
            }
            #p3:before
            {
                content: counters(chapter, ".", lower-roman);
                counter-increment: chapter;
            }
            #p4:before
            {
                content: counters(chapter, ".", upper-roman);
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
