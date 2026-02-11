# css/css-writing-modes/margin-collapse-vrl-016.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vrl-016.xht"
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
      width: 8em;
      writing-mode: vertical-rl;
    }

  div#overflow
    {
      background-color: green;
      margin-right: 4em; /* block-start margin of overflowed block */
      overflow: hidden;
      width: 4em;
    }

  div#nested
    {
      background-color: red;
      margin-right: 4em; /* block-start margin of sub-block */
      width: 4em;
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
