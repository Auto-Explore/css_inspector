# css/css-writing-modes/block-flow-direction-htb-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-htb-001.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: horizontal-tb;
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
      width: 21em;
    }

  div
    {
      background-color: blue;
      border-left: blue solid 1em;
      border-right: blue solid 1em;
    }

  div#top-border
    {
      border-top: blue solid 1em;
    }

  div#bottom-border
    {
      border-bottom: blue solid 1em;
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
