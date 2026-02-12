# css/CSS2/backgrounds/background-position-applies-to-001a-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-applies-to-001a-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  div
  {
  background-color: blue;
  height: 15px;
  margin-left: 93px;

  /*

  From left to right:

     2px : left border-spacing
  +
    25px : border-left of 1st cell
  +
     1px : padding-left of 1st cell
  +
     1px : padding-right of 1st cell
  +
    25px : border-right of 1st cell
  +
     2px : border-spacing between 1st and 2nd cell
  +
    25px : border-left of 2nd cell
  +
     1px : padding-left of 2nd cell
  +
     1px : padding-right of 2nd cell
  +
    25px : border-right of 2nd cell
  -
    15px : width of blue square
  ===============================
    93px

  */

  margin-top: 18px; /* margin-bottom of p + top border-spacing  */
  width: 15px;
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
