# css/css-writing-modes/block-flow-direction-vlr-010.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-vlr-010.xht"
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

  div#abs-pos
    {
      height: 9em;
      left: auto;
      position: absolute;
      writing-mode: vertical-lr;
    }

  div#abs-pos > div
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
