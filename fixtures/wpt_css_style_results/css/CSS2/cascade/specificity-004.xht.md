# css/CSS2/cascade/specificity-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/specificity-004.xht"
}
```

## style[0]

```css

            div:first-child /* a=0 b=0 c=1 d=1 -> specificity = 0,0,1,1 */
            {
                color: green;
            }
            [id=id1] /* a=0 b=0 c=1 d=0 -> specificity = 0,0,1,0 */
            {
                color: red;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
