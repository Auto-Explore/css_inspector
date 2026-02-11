# css/css-writing-modes/block-flow-direction-slr-063.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-slr-063.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: sideways-lr;
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
      border: blue solid 1em;
      border-right: blue none 0em;
    }

  div#right-most
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
