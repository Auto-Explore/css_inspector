# css/css-writing-modes/block-flow-direction-slr-055.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-slr-055.xht"
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

  div#inline-block
    {
      background-color: blue;
      border-bottom: blue solid 1em;
      display: inline-block;
      height: 8em;
      writing-mode: sideways-lr;
    }

  span
    {
      border-left: blue solid 1em;
      display: block;
    }

  span#right-border
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
