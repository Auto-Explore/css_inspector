# css/css-writing-modes/block-flow-direction-vrl-017.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-vrl-017.xht"
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
      writing-mode: vertical-rl;
    }

  div#table-cell > div
    {
      background-color: blue;
      border-bottom: blue solid 1em;
      border-top: blue solid 1em;
    }

  div.right-border
    {
      border-right: blue solid 1em;
    }

  div#left-border
    {
      border-left: blue solid 1em;
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
