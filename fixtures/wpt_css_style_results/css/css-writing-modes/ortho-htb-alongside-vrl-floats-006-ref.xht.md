# css/css-writing-modes/ortho-htb-alongside-vrl-floats-006-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/ortho-htb-alongside-vrl-floats-006-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      height: 100%;
    }

  /*
    minimum used height of the document root element is 176px;
    less than 176px will make the reference file no longer reliable.

       36px : img#pass-fail-conditions-sentence's intrinsic height
     +
        4px : descender space below baseline
     =========
       40px : 1st table (height: 25%)
    multiplied by
        4
     =========
      160px : height of the 3 tables
    +
        8px : body's margin-top
    +
        8px : body's margin-bottom
     =========
      176px
  */

  body
    {
      height: calc(100% - 16px);
    }

  table
    {
      border-spacing: 0px;
      height: 25%;
      margin: 0px;
      table-layout: fixed;
      width: 100%;
    }

  td
    {
      padding: 0px;
      vertical-align: top;
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
