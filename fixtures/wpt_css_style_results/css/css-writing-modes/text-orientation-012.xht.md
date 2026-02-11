# css/css-writing-modes/text-orientation-012.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-012.xht"
}
```

## style[0]

```css
<![CDATA[
  @font-face
    {
      font-family: "mplus-1p-regular";
      src: url("/fonts/mplus-1p-regular.woff") format("woff");
    }

  div
    {
      border: gray solid 1px;
      float: left;
      font-family: "mplus-1p-regular";
      font-size: 32px;
      line-height: 1.5; /* computes to 48px */
      margin-right: 1em;
      text-orientation: upright;
      writing-mode: vertical-rl;
    }

  div#reference
    {
      border-bottom: white none 0px;
      border-right: white none 0px;
    }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “src”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
