# css/CSS2/margin-padding-clear/margin-top-applies-to-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-top-applies-to-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: blue;
  height: 10px;
  }

  div + div
  {
  background-color: orange;
  width: 200px;
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
