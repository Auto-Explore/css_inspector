# css/CSS2/syntax/sgml-comments-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/sgml-comments-002.xht"
}
```

## style[0]

```css
<![CDATA[
    <!--
    .a { color: green; } -->
    <!-- .b { color: green; }
    --> --> --> .c { color: green; }
    <!--
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
