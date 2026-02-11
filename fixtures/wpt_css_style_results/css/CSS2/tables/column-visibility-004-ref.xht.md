# css/CSS2/tables/column-visibility-004-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/column-visibility-004-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: green;
  height: 100px;
  margin-left: 104px;

  /*

     2px : left border-spacing
  +
   100px : width of first cell
  +
     2px : border-spacing between first cell and second cell
  ========
   104px

  */

  margin-top: 18px;
  /* max(p's margin-bottom, top border-spacing + p's margin-bottom) */
  width: 100px;
  }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
