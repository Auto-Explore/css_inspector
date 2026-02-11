# css/CSS2/reference/ref-no-vert-space-between.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/reference/ref-no-vert-space-between.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: blue;
  height: 5px;
  }

  div + div {background-color: orange;}
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
