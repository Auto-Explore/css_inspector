# css/css-writing-modes/background-position-vlr-007.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/background-position-vlr-007.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      background-image: url("support/cat.png");
      background-position: right top;
      background-repeat: repeat-x;
      width: 100%;
      writing-mode: vertical-lr;
    }

  div
    {
      margin-top: 99px;
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
