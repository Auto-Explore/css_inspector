# css/css-writing-modes/block-flow-direction-srl-046.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-srl-046.xht"
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

  div.floated-left
    {
      background-color: blue;
      border-bottom: blue solid 1em;
      border-left: blue solid 1em;
      border-top: blue solid 1em;
      float: left;
      writing-mode: sideways-rl;
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
