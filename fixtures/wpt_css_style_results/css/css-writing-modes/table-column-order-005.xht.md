# css/css-writing-modes/table-column-order-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/table-column-order-005.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      color: yellow;
      font: 20px/1 Ahem;
    }

  table
    {
      background-color: blue;
      border-spacing: 0em;
      border: blue solid 1em;
      height: 7em;
      direction: rtl;
      writing-mode: vertical-lr;
    }

  td
    {
      padding-bottom: 0em;
      padding-left: 1em;
      padding-right: 0em;
      padding-top: 0em;
    }

  td.left-most-cell
    {
      padding-left: 0em;
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
