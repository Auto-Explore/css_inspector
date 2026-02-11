# css/CSS2/css1/c31-important-000.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c31-important-000.xht"
}
```

## style[0]

```css
<![CDATA[
   p { color: green ! important; }
   p { color: red; }
   p#id1 { color: red; }
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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
