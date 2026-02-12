# css/CSS2/cascade/sort-by-order-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/sort-by-order-001.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  height: 100px;
  width: 100px;
  }

  div#parent > div  /* a=0 b=1 c=0 d=2 -> specificity = 0,1,0,2 */
  {
  background-color: red;
  }

  div > div#child /* a=0 b=1 c=0 d=2 -> specificity = 0,1,0,2 */
  {
  background-color: green;
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
