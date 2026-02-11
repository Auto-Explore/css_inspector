# css/CSS2/cascade/specificity-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/specificity-009.xht"
}
```

## style[0]

```css

            html > body > *:first-line /* a=0 b=0 c=0 d=3 -> specificity = 0,0,0,3 */
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
