# css/css-writing-modes/float-contiguous-vlr-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-contiguous-vlr-009.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  div
    {
      height: 200px;
    }

  div > img
    {
      float: right;
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
