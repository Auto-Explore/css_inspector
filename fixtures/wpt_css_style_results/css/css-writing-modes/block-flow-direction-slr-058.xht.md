# css/css-writing-modes/block-flow-direction-slr-058.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-slr-058.xht"
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

  div#table-cell
    {
      display: table-cell;
      height: 9em;
      writing-mode: sideways-lr;
    }

  div#table-cell > div
    {
      background-color: blue;
      border-bottom: blue solid 1em;
      border-top: blue solid 1em;
    }

  div.left-border
    {
      border-left: blue solid 1em;
    }

  div#right-border
    {
      border-right: blue solid 1em;
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
