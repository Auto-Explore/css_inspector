# css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-005-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-005-ref.xht"
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
      position: absolute;
      width: calc(52px + 35px + 50vw + 35px + 52px);
      /*
        52px : offsetWidth of sentence-before
        35px : left border
        50vw : content width of tested orthogonal block
        35px : right border
        52px : offsetWidth of sentence-after
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
      border: blue solid 35px;
      display: block;
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
