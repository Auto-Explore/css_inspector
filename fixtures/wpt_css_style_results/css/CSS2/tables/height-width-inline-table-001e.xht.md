# css/CSS2/tables/height-width-inline-table-001e.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/height-width-inline-table-001e.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red-reference
  {
  background-color: red;
  height: 100px;
  position: absolute;
  width: 100px;
  z-index: -1;
  }

  div#overlapping-green-test
  {
  background-color: green;
  border-bottom: green solid 12px;
  border-left: green solid 14px;
  border-right: green solid 16px;
  border-top: green solid 18px;
  border-collapse: separate;
  display: inline-table;
  height: 33px;
  padding: 20px 22px 17px 15px;
  width: 33px;
  }

  /*
  height of border-box calculation
  --------------------------------

    18px (border-top)
  +
    20px (padding-top)
  +
    33px (content height)
  +
    17px (padding-bottom)
  +
    12px (border-top)
  -----------------------
   100px


  width of border-box calculation
  -------------------------------

    14px (border-left)
  +
    15px (padding-left)
  +
    33px (content width)
  +
    22px (padding-right)
  +
    16px (border-right)
  ----------------------
   100px
  */
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
