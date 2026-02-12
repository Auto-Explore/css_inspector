# css/CSS2/css1/c414-flt-fit-000-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c414-flt-fit-000-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  font: 16px monospace;
  border-collapse: separate;
  border: blue solid medium;
  border-spacing: 0px;
  margin-left: 10px;
  padding: 1em;
  table-layout: fixed;
  width: 16.375em;
  /*
   14em (5em column + 5em column + 4em column)
  + 2em (left and right horizontal padding)
  + 6px (2 vertical 3px borders)
  =====
  16.375em
  */
  }

  col {width: 5em;}

  col#special {width: 4em;}

  td
  {
  padding: 0;
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
