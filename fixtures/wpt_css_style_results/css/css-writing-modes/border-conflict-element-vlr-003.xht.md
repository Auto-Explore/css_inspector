# css/css-writing-modes/border-conflict-element-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-conflict-element-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-collapse: collapse;
      direction: ltr;
      writing-mode: vertical-lr;
    }

  td
    {
      padding: 0px;
    }

  td#five
    {
      border-bottom-color: green;
      border-bottom-style: solid;
      border-bottom-width: 100px;
    }

  td#six
    {
      border-top-color: red;
      border-top-style: solid;
      border-top-width: 100px;
      width: 100px;
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
