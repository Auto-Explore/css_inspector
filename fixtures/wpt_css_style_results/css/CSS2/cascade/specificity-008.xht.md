# css/CSS2/cascade/specificity-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/specificity-008.xht"
}
```

## style[0]

```css

            div:first-child:first-line /* a=0 b=0 c=1 d=2 -> specificity = 0,0,1,2 */
            {
                color: green;
            }
            div:first-line /* a=0 b=0 c=0 d=2 -> specificity = 0,0,0,2 */
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
