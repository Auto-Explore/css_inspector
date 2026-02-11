# css/css-writing-modes/line-box-direction-vlr-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-direction-vlr-008.xht"
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

  div.floated-right
    {
      background-color: blue;
      border-bottom: blue solid 1em;
      border-right: blue solid 1em;
      border-top: blue solid 1em;
      float: right;
      height: 7em; /* Each line box has an inline-size of 7em */
      writing-mode: vertical-lr;
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
