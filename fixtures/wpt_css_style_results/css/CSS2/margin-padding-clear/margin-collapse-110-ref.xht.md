# css/CSS2/margin-padding-clear/margin-collapse-110-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-110-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  background-color: orange;
  border: solid 3px;
  border-spacing: 0px;
  table-layout: fixed;
  width: 256px;
  }

  td {padding: 50px 0px;}

  div#yellow
  {
  border-top: yellow solid 50px;
  margin-bottom: 50px;
  }

  div#lime {border-top: lime solid 50px;}
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
