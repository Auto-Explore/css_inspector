# css/css-writing-modes/horizontal-rule-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/horizontal-rule-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  body
    {
      margin: 0px;
    }

  hr
    {
      background-color: green;
      border: transparent none 0px;
      height: auto;
      margin: 0px;
      width: 10px;
    }

  div#overlapped-red
    {
      background-color: red;
      height: 100%;
      position: relative;
      right: 10px;
      width: 10px;
      z-index: -1;
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
