# css/CSS2/margin-padding-clear/margin-bottom-applies-to-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-bottom-applies-to-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: orange;
  height: 10px;
  margin-top: 216px;
  width: 200px;
  }

  div + div
  {
  background-color: blue;
  margin-top: 0px;
  width: auto;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
