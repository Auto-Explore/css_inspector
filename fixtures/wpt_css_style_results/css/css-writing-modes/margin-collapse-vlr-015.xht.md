# css/css-writing-modes/margin-collapse-vlr-015.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vlr-015.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      font: 25px/1em Ahem;
      height: 4em;
    }

  div#wrapper
    {
      background: url("support/margin-collapse-2em-space-wm-vert.png") -1em top;
      border-left: green solid 1em;
      writing-mode: vertical-lr;
    }

  div#wrapper div
    {
      width: 1em;
    }

  div#overflow
    {
      margin-left: 2em; /* block-start margin of overflowed block */
      overflow: visible;
    }

  div#nested
    {
      background-color: green;
      margin-left: 2em; /* block-start margin of sub-block */
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
