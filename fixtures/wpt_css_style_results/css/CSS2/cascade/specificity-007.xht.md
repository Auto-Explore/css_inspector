# css/CSS2/cascade/specificity-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/specificity-007.xht"
}
```

## style[0]

```css

            div /* a=0 b=0 c=0 d=1 -> specificity = 0,0,0,1 */
            {
                color: green;
            }
            * /* a=0 b=0 c=0 d=0 -> specificity = 0,0,0,0 */
            {
                color: red;
            }
            p
            {
                color: black;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
