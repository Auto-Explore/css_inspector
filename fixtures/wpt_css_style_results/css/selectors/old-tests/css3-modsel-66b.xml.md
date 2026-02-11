# css/selectors/old-tests/css3-modsel-66b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-66b.xml"
}
```

## style[0]

```css
<![CDATA[p { background-color: red; }
p:not(:target) { background-color: lime; }]]>
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
