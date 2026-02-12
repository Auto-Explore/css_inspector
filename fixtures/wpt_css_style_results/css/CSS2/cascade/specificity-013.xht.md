# css/CSS2/cascade/specificity-013.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/specificity-013.xht"
}
```

## style[0]

```css
<![CDATA[
    p { color: yellow; background: red; border: maroon; } /* selects the <p> with 0,0,0,1 */
    div * { color: white; background: green; } /* selects the <p> with 0,0,0,1 and so should override */
    body div * { border: solid lime; } /* selects the <p> with 0,0,0,2 */
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
