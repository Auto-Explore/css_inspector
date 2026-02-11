# css/CSS2/syntax/sgml-comments-000.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/sgml-comments-000.xht"
}
```

## style[0]

```css
<![CDATA[
p {color: red}
<!--
p {color: green}
-->
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
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
