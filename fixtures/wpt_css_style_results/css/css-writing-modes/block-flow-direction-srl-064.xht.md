# css/css-writing-modes/block-flow-direction-srl-064.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-srl-064.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: sideways-rl;
    }
  /*
  "
  The principal writing mode of the document is determined by the writing-mode
  and direction values specified on the root element.
  "
  */

  body
    {
      color: yellow;
      font: 20px/1 Ahem;
      height: 9em;
    }

  div
    {
      background-color: blue;
      border-bottom: blue solid 1em;
      border-right: blue solid 1em;
      border-top: blue solid 1em;
    }

  div#left-border
    {
      border-left: blue solid 1em;
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
