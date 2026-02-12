# css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-007-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-007-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin: 8px 0px;
    }

  table
    {
      border-spacing: 0px;
      width: 310px;
      /*
        52px :
     +
         3px :
     +
       200px :
     +
         3px :
     +
        52px :
    ============
       310px
      */
    }

  td
    {
      padding: 0px;
      vertical-align: top;
    }

  td#after, td#before
    {
      width: 52px;
    }

  p#sentence-after, p#sentence-before
    {
      writing-mode: vertical-lr;
    }

  td#data
    {
      border: blue solid 3px;
      display: block;
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
