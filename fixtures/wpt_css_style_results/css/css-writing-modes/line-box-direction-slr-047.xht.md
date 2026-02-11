# css/css-writing-modes/line-box-direction-slr-047.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-direction-slr-047.xht"
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

  div
    {
      background-color: blue;
      border: blue solid 1em;
      float: right;
      height: 7em; /* Each line box has an inline-size of 7em */
      writing-mode: sideways-lr;
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
